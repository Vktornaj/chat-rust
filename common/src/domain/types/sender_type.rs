use serde::{Deserialize, Serialize};
use super::id::Id;
use uuid::Uuid;


#[derive(Clone, Deserialize, Serialize)]
pub enum Sender {
    User(Id)
}

impl From<Sender> for Uuid {
    fn from(value: Sender) -> Self {
        match value {
            Sender::User(id) => Into::<Uuid>::into(id),
        }
    }
}

impl TryFrom<Uuid> for Sender {
    type Error = String;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match value.try_into() {
            Ok(id) => Ok(Self::User(id)),
            Err(_) => Err("Error converting uuid to id".to_string()),
        }
    }
}

impl From<Id> for Sender {
    fn from(value: Id) -> Self {
        Self::User(value)
    }
}

impl From<Sender> for Id {
    fn from(value: Sender) -> Self {
        match value {
            Sender::User(id) => id,
        }
    }
}