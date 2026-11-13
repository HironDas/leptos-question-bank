use domain::login::Login;
use domain::new_user::NewUser;
use domain::Error;

use crate::common::db;
use crate::common::ServiceContext;
use crate::common::ServiceError;
use crate::user::repository;
use crate::user::repository::User;
use pwhash::bcrypt;

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
        let mut tx = self.context.pool.begin().await.map_err(db)?;
        let user: User = match user_login {
            Login {
                email: Some(email),
                username: None,
                password: _,
            } => repository::get_user_by_email(&mut tx, email.as_ref()).await?,
            Login {
                email: None,
                username: Some(user),
                password: _,
            } => repository::get_user_by_username(&mut tx, user.as_ref()).await?,
            _ => return Err(Error::NotFound.into()),
        };

        if !bcrypt::verify(user_login.password.as_ref(), &user.password_hash) {
            return Err(Error::Invalid("Invalid username or password").into());
        }

        let session = repository::store_session(&mut tx, user.id).await?;
        tx.commit().await.map_err(db)?;
        Ok(session)
    }
    pub async fn signup(&self, user: NewUser) -> Result<(), ServiceError> {
        let mut tx = self.context.pool.begin().await.map_err(db)?;
        let is_user_exist = repository::check_user(&mut tx, user.username.as_ref()).await?;

        let is_email_exist = repository::check_email(&mut tx, user.email.as_ref()).await?;

        if !is_user_exist && !is_email_exist {
            repository::insert_new_user(&mut tx, user).await?;
        } else {
            return Err(anyhow::anyhow!("Username/Email  already exist!!").into());
        }
        Ok(())
    }

    pub async fn is_user_exist(&self, username: String) -> Result<bool, ServiceError> {
        let mut tx = self.context.pool.begin().await.map_err(db)?;
        let exist = repository::check_user(&mut tx, username.as_ref()).await?;
        Ok(exist)
    }

    pub async fn is_email_exist(&self, email: String) -> Result<bool, ServiceError> {
        let mut tx = self.context.pool.begin().await.map_err(db)?;
        let exist = repository::check_email(&mut tx, email.as_ref()).await?;
        Ok(exist)
    }
}
