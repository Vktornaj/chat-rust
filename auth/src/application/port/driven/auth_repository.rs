use std::fmt::Display;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::auth::{Auth, NewAuth};
use crate::domain::types::identification::{IdentificationValue, NewIdentification};


pub enum Error {
    NotFound,
    Unknown(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound => write!(f, "Not found"),
            Error::Unknown(err) => write!(f, "Unknown: {}", err),
        }
    }

}

// #[derive(Default)]
// pub struct UpdateAuth {
//     pub id: Uuid,
//     pub new_hashed_password: Option<String>,
//     pub token_metadata: Option<UpdateTokenMetadata>,
// }

pub struct UpdateTokenMetadata {
    pub id: Uuid,
    pub last_use_timestamp: Option<i64>,
    pub is_active: Option<bool>,
}

pub enum UpdateIdentify<T, U> {
    Add(T),
    Delete(U),
}

#[async_trait]
pub trait AuthRepositoryTrait<T> {
    /// Find and return one single record from the persistence system by id
    async fn find_by_id(&self, conn: &T, user_id: Uuid) -> Result<Auth, Error>;

    async fn find_by_identification(&self, conn: &T, identification: IdentificationValue) -> Result<Option<Auth>, Error>;

    /// Insert the received entity in the persistence system
    async fn create(&self, conn: &T, auth: NewAuth) -> Result<Auth, Error>;

    /// Update one single record already present in the persistence system
    async fn update_password(
        &self, 
        conn: &T, 
        user_id: Uuid, 
        new_hashed_password: String
    ) -> Result<Auth, Error>;
   
    /// Update one single record already present in the persistence system
    async fn update_identifications(
        &self, 
        conn: &T, 
        identification_operation: UpdateIdentify<NewIdentification, Uuid>,
    ) -> Result<Auth, Error>;

    /// Delete one single record from the persistence system
    async fn delete(&self, conn: &T, user_id: Uuid) -> Result<Auth, Error>;
}