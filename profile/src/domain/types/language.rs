use regex::Regex;
use common::domain::types::error::ErrorMsg;
use serde::{Deserialize, Serialize};


// alpha-2 code (ISO 639-1)
#[derive(PartialEq, Debug, Clone, Deserialize, Serialize)]
pub struct Language(String);

impl TryFrom<String> for Language {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Language is empty".to_string()))
        }
        let re = Regex::new(r"^[A-Z]{2}$").unwrap();
        if !re.is_match(&value) {
            return Err(ErrorMsg("Language should be alpha-2 code (ISO 639-1)".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<Language> for String {
    fn from(value: Language) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests_language {
    use super::*;

    #[test]
    fn test_language() {
        let language = Language::try_from("AA".to_owned());
        assert!(language.is_ok());
        let language = Language::try_from("A".to_owned());
        assert!(language.is_err());
        let language = Language::try_from("A2".to_owned());
        assert!(language.is_err());
        let language = Language::try_from("AA2".to_owned());
        assert!(language.is_err());
    }
}