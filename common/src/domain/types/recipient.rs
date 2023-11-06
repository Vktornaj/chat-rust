use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{id::Id, group::Group};


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