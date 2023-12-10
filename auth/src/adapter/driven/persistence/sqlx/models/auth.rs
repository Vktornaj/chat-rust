use chrono::{DateTime, Utc};
use common::domain::types::{error::ErrorMsg, id::Id, email::Email, phone_number::PhoneNumber};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::domain::{auth::Auth, types::{identification::{Identification, IdentificationValue, self}, token_metadata::TokenMetadata}};


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
        identifications: Vec<IdentificationSQL>,
        tokens_metadata: Vec<TokenMetadataSQL>,
    ) -> Result<Auth, ErrorMsg> {
        let identifications = identifications.into_iter().map(|identification| {
            identification.to_identification_domain()
        }).collect::<Result<Vec<Identification>, ErrorMsg>>()?;
        let tokens_metadata = tokens_metadata.into_iter().map(|token_metadata| {
            token_metadata.to_token_metadata_domain()
        }).collect::<Result<Vec<TokenMetadata>, ErrorMsg>>()?;
        Ok(Auth {
            user_id: Id::try_from(self.user_id)?,
            identifications,
            hashed_password: self.hashed_password,
            tokens_metadata,
            created_at: self.created_at,
            updated_at: self.updated_at
        })
    }
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
    pub fn to_identification_domain(self) -> Result<Identification, ErrorMsg> {

        let identification_value = match self.identification_type.as_str() {
            "email" => IdentificationValue::Email(Email::try_from(self.identification_value)?),
            "phone_number" => IdentificationValue::PhoneNumber(PhoneNumber::try_from(self.identification_value)?),
            _ => return Err(ErrorMsg("Invalid identification type".to_string())),
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
    pub user_id: Uuid,
    pub creation_timestamp: i64,
    pub last_use_timestamp: i64,
    pub is_active: bool,
    pub browser: String,
    pub os: String,
}

impl TokenMetadataSQL {
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