use domain::login::Login;
use domain::new_user::NewUser;
use domain::Error;
use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::common::ServiceContext;
use crate::common::ServiceError;
use crate::user::repository;
use crate::user::repository::User;
use pwhash::bcrypt;

type HmacSha256 = Hmac<Sha256>;

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
    pub async fn login(&self, user_login: Login) -> Result<String, ServiceError> {
        let pool = &self.context.pool;
        let user: User = match &user_login {
            Login {
                email: Some(email),
                username: None,
                password: _,
            } => repository::get_user_by_email(pool, email.as_ref()).await?,
            Login {
                email: None,
                username: Some(user),
                password: _,
            } => repository::get_user_by_username(pool, user.as_ref()).await?,
            _ => return Err(Error::Invalid("Provide eithter email or username").into()),
        };

        if !bcrypt::verify(user_login.password.as_ref(), &user.password_hash) {
            return Err(Error::Invalid("Invalid username or password").into());
        }

        let session = repository::store_session(&pool, user.id).await?;
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

    fn rate_limit_key(
        secret: &[u8],
        scope: &str,
        identifier: &str,
    ) -> Result<[u8; 32], ServiceError> {
        let mut mac = HmacSha256::new_from_slice(secret)
            .map_err(|_| anyhow::anyhow!("Valid HMAC key"))?;
        mac.update(scope.as_bytes());
        mac.update(b":");
        mac.update(identifier.as_bytes());

        Ok(mac.finalize().into_bytes().into())
    }
}
