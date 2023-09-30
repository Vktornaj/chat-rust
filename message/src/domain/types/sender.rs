use serde::{Deserialize, Serialize};
use user::types::id::Id;
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
