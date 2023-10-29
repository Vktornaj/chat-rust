use serde::{Serialize, Deserialize};
use uuid::Uuid;
use super::{error::ErrorMsg, recipient::Recipient};


#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
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

impl From<Id> for Uuid {
    fn from(id: Id) -> Self {
        id.0
    }
}

impl From<Recipient> for Id {
    fn from(value: Recipient) -> Self {
        match value {
            Recipient::User(id) => id,
            Recipient::Group(id) => id,
        }
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
