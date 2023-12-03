use chrono::{DateTime, Utc};

use common::domain::types::id::Id;
use super::types::{
    token_metadata::TokenMetadata, 
    identification::Identification,
};


pub struct Auth {
    pub user_id: Id,
    pub hashed_password: String,
    pub tokens_metadata: Vec<TokenMetadata>,
    pub identifications: Vec<Identification>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NewAuth {
    pub hashed_password: String,
    pub identifications: Vec<Identification>,
}
