use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{group::Group, id::Id};


#[derive(Clone, Deserialize, Serialize)]
pub enum Recipient {
    User(Id),
    Group(Group),
}

impl TryFrom<Uuid> for Recipient {
    type Error = &'static str;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        match Id::try_from(value) {
            Ok(id) => Ok(Self::User(id)),
            Err(_) => Err("Invalid id"),
        }
    }
}

impl PartialEq for Recipient {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::User(id1), Self::User(id2)) => id1 == id2,
            (Self::Group(group1), Self::Group(group2)) => group1 == group2,
            _ => false,
        }
    }
}

impl PartialEq<Id> for Recipient {
    fn eq(&self, other: &Id) -> bool {
        match (self, other) {
            (Self::User(id1), id2) => id1 == id2,
            _ => false,
        }
    }
}

impl From<Recipient> for String {
    fn from(value: Recipient) -> Self {
        match value {
            Recipient::User(id) => Into::<String>::into(id),
            Recipient::Group(group) => Into::<String>::into(group.id),
        }
    }
}