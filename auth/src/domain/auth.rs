use chrono::{DateTime, Utc};

use common::domain::types::{id::Id, error::ErrorMsg};
use super::{
    types::{token_metadata::TokenMetadata, password::Password}, 
    contact::ContactDetails,
};


pub struct Auth {
    pub user_id: Id,
    pub hashed_password: String,
    pub tokens_metadata: Vec<TokenMetadata>,
    pub contact_details: Vec<ContactDetails>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewAuth {
    pub password: String,
    pub contact_details: Vec<ContactDetails>,
}

impl NewAuth {
    pub fn new(
        password: String,
        contact_details: Vec<ContactDetails>,
    ) -> Result<Self, ErrorMsg> {
        let hashed_password = match Password::try_from(password)?.hash_password() {
            Ok(hashed_password) => hashed_password,
            Err(e) => return Err(ErrorMsg(e.to_string())),
        };
        Ok(NewAuth { password: hashed_password, contact_details })
    }
}