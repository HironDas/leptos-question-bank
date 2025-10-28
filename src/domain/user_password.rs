use serde::{Deserialize, Serialize};
use validator::ValidationError;

#[derive(Debug, Clone, Eq, Serialize, Deserialize)]
pub struct UserPassword(String);

impl UserPassword {
    pub fn parse(password: String) -> Result<Self, ValidationError> {
        match validate_password(&password) {
            Ok(_) => Ok(Self(password)),
            Err(err) => Err(err),
        }
    }
}

fn validate_password(password: &str) -> Result<(), ValidationError> {
    let has_upper = password.chars().any(|c| c.is_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    let has_special = password.chars().any(|c| !c.is_alphanumeric());
    let has_lower = password.chars().any(|c| c.is_lowercase());

    if password.len() < 8
        || password.len() > 16
        || !has_upper
        || !has_digit
        || !has_special
        || !has_lower
    {
        return Err(ValidationError::new("INVALID_PASSWORD")
            .with_message("Include At least one special character, one uppercase letter, and one digit & minimum 8-16 characters".into()));
    }

    Ok(())
}

impl AsRef<str> for UserPassword {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl PartialEq for UserPassword {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn test_parse_valid_password() {
        let password = UserPassword::parse("Abc123!e".to_string()).unwrap();
        assert_eq!(password.as_ref(), "Abc123!e");
    }

    #[test]
    fn test_parse_invalid_password() {
        let result = UserPassword::parse("abc123".to_string());
        assert!(result.is_err());
    }

    // prop test
    fn safe_password_strategy() -> impl Strategy<Value = String> {
        use rand::seq::SliceRandom; // Bring the trait into scope

        // Collect all character sets into Vecs so they can be cloned as needed
        let lowercase: Vec<char> = ('a'..='z').collect();
        let uppercase: Vec<char> = ('A'..='Z').collect();
        let digits: Vec<char> = ('0'..='9').collect();
        let special: Vec<char> = "!@#$%^&*()_+=-".chars().collect();

        let required = (
            prop::sample::select(lowercase.clone()),
            prop::sample::select(uppercase.clone()),
            prop::sample::select(digits.clone()),
            prop::sample::select(special.clone()),
        );

        let all_chars: Vec<char> = lowercase
            .iter()
            .chain(uppercase.iter())
            .chain(digits.iter())
            .chain(special.iter())
            .copied()
            .collect();

        let filler = prop::collection::vec(
            prop::sample::select(all_chars.clone()),
            4..13, // total length will be at least 8
        );

        (required, filler).prop_map(|((l, u, d, s), mut rest)| {
            let mut all = vec![l, u, d, s];
            all.append(&mut rest);
            let mut rng = rand::thread_rng(); // Use the correct function to get a thread-local RNG
            all.shuffle(&mut rng);
            all.into_iter().collect()
        })
    }
    proptest! {
        #[test]
        fn test_parse_valid_password_proptest(safe_password in safe_password_strategy()) {
            let password = UserPassword::parse(safe_password.clone()).unwrap();
            assert_eq!(password.as_ref(), safe_password);
        }
    }
}
