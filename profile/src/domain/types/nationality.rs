use regex::Regex;
use common::domain::types::error::ErrorMsg;
use serde::{Deserialize, Serialize};



// alpha-3 code (ISO 3166)
#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct Nationality(String);

impl TryFrom<String> for Nationality {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Nationality is empty".to_string()))
        }
        let re = Regex::new(r"^[A-Z]{3}$").unwrap();
        if !re.is_match(&value) {
            return Err(ErrorMsg("Nationality should be alpha-3 code (ISO 3166)".to_string()))
        }
        Ok(Self(value))
    }
}

impl<'de> Deserialize<'de> for Nationality {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let nationality = String::deserialize(deserializer)?;
        Self::try_from(nationality).map_err(serde::de::Error::custom)
    }
}


impl From<Nationality> for String {
    fn from(value: Nationality) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests_nationality {
    use super::*;

    #[test]
    fn test_nationality() {
        let nationality = Nationality::try_from("AAA".to_owned());
        assert!(nationality.is_ok());
        let nationality = Nationality::try_from("AA".to_owned());
        assert!(nationality.is_err());
        let nationality = Nationality::try_from("A".to_owned());
        assert!(nationality.is_err());
        let nationality = Nationality::try_from("A2".to_owned());
        assert!(nationality.is_err());
        let nationality = Nationality::try_from("AAA2".to_owned());
        assert!(nationality.is_err());
    }
}
