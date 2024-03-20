use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

use common::domain::types::id::Id;
use super::types::alias::Alias;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    pub id: Id,
    pub alias: Option<Alias>,
    pub is_blocked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct NewContact {
    pub id: Id,
    pub user_id: Id,
    pub alias: Option<Alias>,
    pub is_blocked: bool,
}
