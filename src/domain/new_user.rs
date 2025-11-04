use crate::domain::{user_email::UserEmail, user_password::UserPassword, user_username::Username};
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Validate, Deserialize, Serialize)]
pub struct NewUser {
    pub username: Username,
    pub email: UserEmail,
    pub password: UserPassword,
    #[validate(must_match(
        other = "password",
        message = "Passwords do not match",
        code = "Unauthorized"
    ))]
    pub confirm_password: UserPassword,
}

impl NewUser {
    pub fn hash_password(&self) -> String {
        bcrypt::hash(self.password.as_ref()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_user_validation() {
        let new_user = NewUser {
            username: Username::parse("test_user".to_string()).unwrap(),
            email: UserEmail::parse("test@example.com".to_string()).unwrap(),
            password: UserPassword::parse("Hiron@12345".to_string()).unwrap(),
            confirm_password: UserPassword::parse("Hiron@12345".to_string()).unwrap(),
        };

        assert!(new_user.validate().is_ok());
    }
}
