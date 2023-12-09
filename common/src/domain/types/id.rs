use std::fmt::Display;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::error::ErrorMsg;


#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Copy)]
pub struct Id(Uuid);

impl TryFrom<Uuid> for Id {
    type Error = ErrorMsg;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        if value.is_nil() {
            return Err(ErrorMsg("Id is nil".to_string()))
        }
        Ok(Self(value))
    }
}

impl TryFrom<String> for Id {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = Uuid::parse_str(&value).map_err(|err| ErrorMsg(err.to_string()))?;
        Self::try_from(value)
    }
}

impl From<Id> for Uuid {
    fn from(id: Id) -> Self {
        id.0
    }
}

impl From<Id> for String {
    fn from(id: Id) -> Self {
        id.0.to_string()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.to_string().fmt(f)
    }
}


#[cfg(test)]
mod tests_id {
    use super::*;

    #[test]
    fn test_id() {
        let id = Id::try_from(Uuid::new_v4());
        assert!(id.is_ok());
        let id = Id::try_from(Uuid::nil());
        assert!(id.is_err());
    }
}
