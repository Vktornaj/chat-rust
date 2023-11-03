use serde::{Deserialize, Serialize};

use super::id::Id;
use super::group::Group;


#[derive(Clone, Deserialize, Serialize)]
pub enum Recipient {
    User(Id),
    Group(Group),
}