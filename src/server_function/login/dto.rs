use domain::{login::Login, user_email::UserEmail, user_password::UserPassword};
use infra::common::ServiceError;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct LoginCredential {
    pub id: String,
    pub password: String,
}

impl TryFrom<LoginCredential> for Login {
    type Error = ServiceError;

    fn try_from(value: LoginCredential) -> Result<Self, Self::Error> {
        use domain::user_username::Username as Name;
        let password = UserPassword::parse(value.password)?;
        match UserEmail::parse(value.id.clone()) {
            Ok(email) => Ok(Login {
                username: None,
                email: Some(email),
                password,
            }),
            Err(_) => match Name::parse(value.id) {
                Ok(username) => Ok(Login {
                    username: Some(username),
                    email: None,
                    password,
                }),
                Err(err) => Err(ServiceError::ValidationError(err)),
            },
        }
    }
}
