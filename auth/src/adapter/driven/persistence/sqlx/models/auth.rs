use chrono::{DateTime, Utc};
use common::domain::types::{error::ErrorMsg, id::Id, email::Email, phone_number::PhoneNumber};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::domain::{auth::Auth, types::{identification::{Identification, IdentificationValue}, token_metadata::TokenMetadata}};


pub struct AuthSQL {
    pub user_id: Uuid,
    pub hashed_password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl AuthSQL {
    pub fn from_pgrow(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user_id: row.try_get("user_id")?,
            hashed_password: row.try_get("hashed_password")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    pub fn to_auth_domain(
        self, 
        identifications: Vec<Identification>,
        tokens_metadata: Vec<TokenMetadata>,
    ) -> Result<Auth, ErrorMsg> {
        Ok(Auth {
            user_id: Id::try_from(self.id)?,
            identifications,
            hashed_password: self.hashed_password,
            tokens_metadata,
            created_at: self.created_at,
            updated_at: self.updated_at
        })
    }
} 

pub enum IdentificationSQLType {
    Email,
    PhoneNumber,
}

pub struct IdentificationSQL {
    pub id: Uuid,
    pub user_id: Uuid,
    pub identification_type: String,
    pub identification_value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl IdentificationSQL {
    pub fn from_pgrow(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            user_id: row.try_get("user_id")?,
            identification_type: row.try_get("identification_type")?,
            identification_value: row.try_get("identification_value")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    pub fn to_identification_domain(self) -> Result<Identification, ErrorMsg> {

        let identification_value = match self.identification_type.as_str() {
            "email" => IdentificationValue::Email(Email::try_from(self.identification_value)?),
            "phone_number" => IdentificationValue::PhoneNumber(PhoneNumber::try_from(self.identification_value)?),
            _ => return Err(ErrorMsg::new("Invalid identification type")),
        };

        Ok(Identification {
            id: Id::try_from(self.id)?,
            user_id: Id::try_from(self.user_id)?,
            identification_value,
            created_at: self.created_at,
        })
    }
}

pub struct TokenMetadataSQL {
    pub token_id: Uuid,
    pub creation_timestamp: i64,
    pub last_use_timestamp: i64,
    pub is_active: bool,
    pub browser: String,
    pub os: String,
}

impl TokenMetadataSQL {
    pub fn from_pgrow(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            token_id: row.try_get("token_id")?,
            creation_timestamp: row.try_get("creation_timestamp")?,
            last_use_timestamp: row.try_get("last_use_timestamp")?,
            is_active: row.try_get("is_active")?,
            browser: row.try_get("browser")?,
            os: row.try_get("os")?,
        })
    }

    pub fn to_token_metadata_domain(self) -> Result<TokenMetadata, ErrorMsg> {
        Ok(TokenMetadata {
            token_id: self.token_id,
            creation_timestamp: self.creation_timestamp,
            last_use_timestamp: self.last_use_timestamp,
            is_active: self.is_active,
            browser: self.browser,
            os: self.os,
        })
    }
}