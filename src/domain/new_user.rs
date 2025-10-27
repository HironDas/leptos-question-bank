use crate::domain::{user_email::UserEmail, user_password::UserPassword, user_username::Username};
use pwhash::bcrypt;
use validator::Validate;

#[derive(Debug, Clone, PartialEq, Eq, Validate)]
pub struct NewUser {
    pub username: Username,
    pub email: UserEmail,
    pub password: UserPassword,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
    pub confirm_password: UserPassword,
}

impl NewUser {
    pub fn new(
        username: String,
        email: String,
        password: String,
        confirm_password: String,
    ) -> Self {
        NewUser {
            username: Username::parse(username).unwrap(),
            email: UserEmail::parse(email).unwrap(),
            password: UserPassword::parse(password).unwrap(),
            confirm_password: UserPassword::parse(confirm_password).unwrap(),
        }
    }

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
