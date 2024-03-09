use chrono::{DateTime, Utc};

use common::domain::types::id::Id;
use super::types::alias::Alias;


pub struct Contact {
    pub id: Id,
    pub alias: Option<Alias>,
    pub is_blocked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewContact {
    pub id: Id,
    pub user_id: Id,
    pub alias: Option<Alias>,
    pub blocked: bool,
}
