use domain::login::Login;
use domain::Error;
use domain::Result;

use crate::common::db;
use crate::common::ServiceContext;
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
    pub async fn login(&self, user_login: Login) -> Result<i32> {
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
            _ => return Err(Error::NotFound),
        };

        tx.commit().await.map_err(db)?;

        if bcrypt::verify(user_login.password.as_ref(), &user.password_hash) {
            Ok(user.id)
        } else {
            Err(Error::Invalid(
                "Invalid username or password, please try again",
            ))
        }
    }
}
