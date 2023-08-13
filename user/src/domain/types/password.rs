use regex::Regex;
use argon2::{
    password_hash::{
        PasswordHash, PasswordVerifier, Error, SaltString, rand_core::OsRng
    },
    Argon2, PasswordHasher
};

use super::error::ErrorMsg;


#[derive(PartialEq, Debug, Clone)]
pub struct Password(String);

impl TryFrom<String> for Password {
    type Error = ErrorMsg;

    // TODO: test these regex
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Password is empty".to_string()))
        }
        if value.len() < 8 || value.len() > 64 {
            return Err(ErrorMsg("Password length should be between 8 and 64".to_string()))
        }
        // At least one uppercase
        if !Regex::new(r"[A-Z]").unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should contain at least one uppercase".to_string()))
        }
        // At least one lowercase
        if !Regex::new(r"[a-z]").unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should contain at least one lowercase".to_string()))
        }
        // At least one digit
        if !Regex::new(r"[0-9]").unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should contain at least one digit".to_string()))
        }
        // At least one special character
        if !Regex::new(r#"[!@#$%^&*(),.?\":{}|<>]"#).unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should contain at least one special character".to_string()))
        }
        // No whitespace
        if Regex::new(r"\s").unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should not contain whitespace".to_string()))
        }
        // No unicode
        if Regex::new(r"[^\x00-\x7F]").unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should not contain unicode".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<Password> for String {
    fn from(password: Password) -> Self {
        password.0
    }
}

impl Password {   
    // TODO: Reduce the runtime; 1.2 seconds
    pub fn verify_password(self, hashed_password: &String) -> Result<(), Error> {
        let parsed_hash = PasswordHash::new(hashed_password)?;
        Argon2::default().verify_password(
            Into::<String>::into(self).as_bytes(), 
            &parsed_hash
        )
    }
    
    // TODO: Reduce the runtime; 1.3 seconds
    pub fn hash_password(self) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);
        
        let argon2 = Argon2::default();
        Ok(argon2.hash_password(Into::<String>::into(self).as_bytes(), &salt)?.to_string())
    }
}

#[cfg(test)]
mod tests_password {
    use super::*;

    #[test]
    fn test_password() {
        // ok
        let password = Password::try_from("Passwo1!".to_string());
        assert!(password.is_ok());
        let password = Password::try_from("Password123%".to_string());
        assert!(password.is_ok());
        let password = Password::try_from("@PASSWORD1p,".to_string());
        assert!(password.is_ok());
        // error
        let password = Password::try_from("Password1".to_string());
        assert!(password.is_err());
        let password = Password::try_from("password1!".to_string());
        assert!(password.is_err());
        let password = Password::try_from("Password!".to_string());
        assert!(password.is_err());
        let password = Password::try_from("Pss1!".to_string());
        assert!(password.is_err());
    }
}