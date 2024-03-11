
use std::fmt::Display;
use serde::Deserialize;

#[derive(Debug)]
pub enum Error {
    TooShort,
    TooLong,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::TooShort => write!(f, "Alias is too short"),
            Error::TooLong => write!(f, "Alias is too long"),
        }
    }
}

pub struct Alias(String);

impl TryFrom<String> for Alias {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() < 3 {
            return Err(Error::TooShort);
        }
        if value.len() > 50 {
            return Err(Error::TooLong);
        }
        Ok(Alias(value))
    }
}

impl From<Alias> for String {
    fn from(alias: Alias) -> Self {
        alias.0
    }
}

impl<'de> Deserialize<'de> for Alias {
    fn deserialize<D>(deserializer: D) -> Result<Alias, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Alias::try_from(s).map_err(serde::de::Error::custom)
    }
}
