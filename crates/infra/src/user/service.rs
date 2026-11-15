use std::net::IpAddr;

use anyhow::anyhow;
use domain::login::Login;
use domain::new_user::NewUser;
use domain::Error;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::types::Uuid;

use crate::common::ServiceContext;
use crate::common::ServiceError;
use crate::rate_limit;
use crate::user::repository;
use crate::user::repository::User;
use pwhash::bcrypt;

type HmacSha256 = Hmac<Sha256>;
const DUMMY_BCRYPT_HASH: &str = "$2b$10$IBBT1ZKMUhro6XJabnran.UpCG7yrqQxSxGSI5bDnfmAC/Xm9nxpm";

#[derive(Clone)]
pub struct UserService {
    pub(crate) context: ServiceContext,
}

impl From<ServiceContext> for UserService {
    fn from(context: ServiceContext) -> Self {
        Self { context }
    }
}

impl UserService {
    pub async fn login(
        &self,
        user_login: Login,
        client_ip: IpAddr,
    ) -> Result<String, ServiceError> {
        let pool = &self.context.pool;
        let id;
        let ip_key = Self::rate_limit_key(
            &*self.context.rate_limit_secret,
            "login:ip",
            &client_ip.to_string(),
        )?;

        rate_limit::repository::reserve_login_attempt(pool, &ip_key, 20, 60)
            .await?
            .then_some(())
            .ok_or(Error::RateLimited)?;

        let user: domain::Result<User> = match &user_login {
            Login {
                email: Some(email),
                username: None,
                password: _,
            } => {
                id = email.as_ref();
                repository::get_user_by_email(pool, email.as_ref()).await
            }
            Login {
                email: None,
                username: Some(user),
                password: _,
            } => {
                id = user.as_ref();
                repository::get_user_by_username(pool, user.as_ref()).await
            }
            _ => return Err(Error::Invalid("Provide eithter email or username").into()),
        };

        let key =
            Self::rate_limit_key(&*self.context.rate_limit_secret.clone(), "login:acount", id)?;

        rate_limit::repository::reserve_login_attempt(pool, &key, 5, 900)
            .await?
            .then_some(())
            .ok_or(Error::RateLimited)?;

        let user = match user {
            Ok(user) => Some(user),
            Err(Error::NotFound) => None,
            Err(err) => return Err(err.into()),
        };

        let hash = user
            .as_ref()
            .map(|user| user.password_hash.as_str())
            .unwrap_or(DUMMY_BCRYPT_HASH);
        if !Self::password_verify(user_login.password.as_ref(), &hash).await? {
            return Err(Error::Invalid("Invalid username or password").into());
        }

        let session = repository::store_session(pool, user.unwrap().id).await?;
        // tx.commit().await.map_err(db)?;
        Ok(session)
    }
    pub async fn signup(&self, user: NewUser) -> Result<(), ServiceError> {
        repository::insert_new_user(&self.context.pool, user)
            .await
            .map_err(Into::into)
    }

    pub async fn is_user_exist(&self, username: String) -> Result<bool, ServiceError> {
        let pool = &self.context.pool;
        let exist = repository::check_user(pool, username.as_ref()).await?;
        Ok(exist)
    }

    pub async fn is_email_exist(&self, email: String) -> Result<bool, ServiceError> {
        let pool = &self.context.pool;
        let exist = repository::check_email(pool, email.as_ref()).await?;
        Ok(exist)
    }

    pub async fn logout(&self, session_token: &str) -> Result<(), ServiceError> {
        let pool = &self.context.pool;
        let session_id =
            Uuid::parse_str(session_token).map_err(|_| anyhow!("Failed to parse token"))?;

        repository::delete_session_on_logout(pool, session_id).await?;

        Ok(())
    }

    fn rate_limit_key(
        secret: &[u8; 32],
        scope: &str,
        identifier: &str,
    ) -> Result<[u8; 32], ServiceError> {
        let mut mac =
            HmacSha256::new_from_slice(secret).map_err(|_| anyhow::anyhow!("Valid HMAC key"))?;
        mac.update(scope.as_bytes());
        mac.update(b":");
        mac.update(identifier.as_bytes());

        Ok(mac.finalize().into_bytes().into())
    }

    async fn password_verify(pass: &str, hash: &str) -> domain::Result<bool> {
        let pass = pass.to_owned();
        let hash = hash.to_owned();
        tokio::task::spawn_blocking(move || bcrypt::verify(&pass, &hash))
            .await
            .map_err(|_| Error::Internal)
    }
}
