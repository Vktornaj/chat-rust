use serde::{Serialize, Deserialize};

use super::id::Id;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Group {
    pub id: Id,
    pub name: String,
    pub members: Vec<Id>,
}