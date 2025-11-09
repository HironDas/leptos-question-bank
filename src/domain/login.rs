use crate::domain::{user_email::UserEmail, user_password::UserPassword, user_username::Username};

pub struct Login {
    pub email: Option<UserEmail>,
    pub username: Option<Username>,
    pub password: UserPassword,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_creation() {
        let email = UserEmail::parse("test@example.com".to_string()).unwrap();
        let username = Username::parse("testuser".to_string()).unwrap();
        let password = UserPassword::parse("Pass@123!".to_string()).unwrap();

        let login = Login {
            email: Some(email),
            username: Some(username),
            password,
        };

        assert!(login.email.is_some());
        assert!(login.username.is_some());
    }
}
