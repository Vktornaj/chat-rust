use serde::{Deserialize, Serialize};
use user::types::id::Id;
use uuid::Uuid;


#[derive(Clone, Deserialize, Serialize)]
pub enum Recipient {
    User(Id),
    Group(Id),
}

impl From<Recipient> for Uuid {
    fn from(value: Recipient) -> Self {
        match value {
            Recipient::User(id) => id.into(),
            Recipient::Group(id) => id.into(),
        }
    }
}