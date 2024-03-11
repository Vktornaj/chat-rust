use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::contact::Contact;


pub struct ContactDB {
    pub id: Uuid,
    pub user_id: Uuid,
    pub alias: Option<String>,
    pub is_blocked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl ContactDB {
    pub fn to_contact(self) -> Contact {
        Contact {
            id: self.id.try_into().unwrap(),
            alias: self.alias.map(|x| x.try_into().unwrap()),
            is_blocked: self.is_blocked,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }   
}