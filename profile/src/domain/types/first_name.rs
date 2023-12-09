use regex::Regex;
use serde::{Deserialize, Serialize};
use common::domain::types::error::ErrorMsg;


#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct FirstName(String);

impl TryFrom<String> for FirstName {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("First name is empty".to_string()))
        }
        if value.len() > 50 {
            return Err(ErrorMsg("First name is too long, max length 50".to_string()))
        }
        // at least one letter
        if !Regex::new(r"[a-zA-Z]").unwrap().is_match(&value) {
            return Err(ErrorMsg("First name should contain at least one letter".to_string()))
        }
        // only letters and spaces
        if !Regex::new(r"^[a-zA-Z ]+$").unwrap().is_match(&value) {
            return Err(ErrorMsg("First name should contain only letters and spaces".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<FirstName> for String {
    fn from(first_name: FirstName) -> Self {
        first_name.0
    }
}

#[cfg(test)]
mod tests_first_name {
    use super::*;

    #[test]
    fn test_first_name() {
        let first_name = FirstName::try_from("Some".to_string());
        assert!(first_name.is_ok());
        let first_name = FirstName::try_from("Some Somes".to_string());
        assert!(first_name.is_ok());
        let first_name = FirstName::try_from("Some1".to_string());
        assert!(first_name.is_err());
    }
}