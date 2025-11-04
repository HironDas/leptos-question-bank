use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct Username(String);

impl Username {
    pub fn parse(input: String) -> Result<Self, ValidationError> {
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}', '\''];
        let contains_forbidden_characters =
            input.chars().any(|g| forbidden_characters.contains(&g));
        if input.len() < 3
            || input.contains(' ')
            || input.len() > 20
            || contains_forbidden_characters
        {
            Err(ValidationError::new("UNPROCESSABLE_ENTITY")
                .with_message("Username cannot be less than 3 characters and no spaces".into()))
        } else {
            Ok(Username(input))
        }
    }
}

impl AsRef<str> for Username {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use fake::{faker::name::raw::*, locales::EN, Fake};

    use super::*;

    #[test]
    fn test_username_parse() {
        let fack_users: Vec<String> = (FirstName(EN), 3..20).fake();
        fack_users.into_iter().for_each(|user| {
            println!("username: {}", user);
            assert!(Username::parse(user).is_ok());
        });
        assert_eq!(
            Username::parse("john".to_string()),
            Ok(Username("john".to_string()))
        );
        assert_eq!(
            Username::parse("j".to_string()),
            Err(ValidationError::new("INVALID_USERNAME")
                .with_message("Username cannot be less than 3 characters and no spaces".into()))
        );
        assert_eq!(
            Username::parse("john doe".to_string()),
            Err(ValidationError::new("INVALID_USERNAME")
                .with_message("Username cannot be less than 3 characters and no spaces".into()))
        );
    }
}
