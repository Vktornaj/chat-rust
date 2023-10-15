use regex::Regex;
use common::types::error::ErrorMsg;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct LastName(String);

impl TryFrom<String> for LastName {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Last name is empty".to_string()))
        }
        if value.len() > 50 {
            return Err(ErrorMsg("Last name is too long, max length 50".to_string()))
        }
        // at least one letter
        if !Regex::new(r"[a-zA-Z]").unwrap().is_match(&value) {
            return Err(ErrorMsg("Last name should contain at least one letter".to_string()))
        }
        // only letters and spaces
        if !Regex::new(r"^[a-zA-Z ]+$").unwrap().is_match(&value) {
            return Err(ErrorMsg("Last name should contain only letters and spaces".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<LastName> for String {
    fn from(last_name: LastName) -> Self {
        last_name.0
    }
}

#[cfg(test)]
mod tests_last_name {
    use super::*;

    #[test]
    fn test_last_name() {
        let last_name = LastName::try_from("Some".to_string());
        assert!(last_name.is_ok());
        let last_name = LastName::try_from("Some Somes".to_string());
        assert!(last_name.is_ok());
        let last_name = LastName::try_from("Some1".to_string());
        assert!(last_name.is_err());
    }
}
