use domain::login::Login;
use domain::Result;

use crate::common::ServiceContext;

#[derive(Clone)]
pub struct UserService {
    pub(crate) context: ServiceContext,
}

impl From<ServiceContext> for UserService {
    fn from(context: ServiceContext) -> Self {
        Self { context }
    }
}

impl UserService{
    pub async login(&self, user_login: Login) -> Result<i32>{
        Ok(1)
    }
}
