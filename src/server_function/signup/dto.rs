use domain::{
    new_user::NewUser, user_email::UserEmail, user_password::UserPassword, user_username::Username,
};
use infra::common::ServiceError;
use validator::{Validate, ValidationError};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

impl TryFrom<User> for NewUser {
    type Error = ServiceError;
    fn try_from(value: User) -> Result<Self, Self::Error> {
        let username = Username::parse(value.username).map_err(|err| {
            println!("{}:{}", err.code, err.message.as_ref().unwrap());
            err
        })?;
        let email = UserEmail::parse(value.email).map_err(|err| {
            println!("{}:{}", err.code, err.message.as_ref().unwrap());
            err
        })?;
        let password = UserPassword::parse(value.password).map_err(|err| {
            println!("{}:{}", err.code, err.message.as_ref().unwrap());
            err
        })?;
        let confirm_password = UserPassword::parse(value.confirm_password).map_err(|err| {
            println!("{}:{}", err.code, err.message.as_ref().unwrap());
            err
        })?;

        let new_user = NewUser {
            username,
            email,
            password,
            confirm_password,
        };

        match new_user.validate() {
            Ok(_) => Ok(new_user),
            Err(errs) => {
                let mut first_error = None;
                errs.field_errors().into_iter().for_each(|(field, errors)| {
                    errors.into_iter().for_each(|error| {
                        println!("{}: {}", field, error.message.as_ref().unwrap());
                    });
                    first_error = errors.into_iter().next();
                });

                if let Some(error) = first_error {
                    Err(error.clone().into())
                } else {
                    Err(ValidationError::new("Unknown error").into())
                }
            }
        }
    }
}
