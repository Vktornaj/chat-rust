use serde::{Deserialize, Serialize};
use super::id::Id;
use uuid::Uuid;


#[derive(Clone, Deserialize, Serialize)]
pub enum Sender {
    User(Id),
    Group(Uuid),
}

impl From<Sender> for Uuid {
    fn from(value: Sender) -> Self {
        match value {
            Sender::User(id) => Into::<Uuid>::into(id),
            Sender::Group(name) => name,
        }
    }
}

impl TryFrom<Uuid> for Sender {
    type Error = &'static str;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        Ok(Self::Group(value))
    }
}

impl From<Id> for Sender {
    fn from(value: Id) -> Self {
        Self::User(value)
    }
}