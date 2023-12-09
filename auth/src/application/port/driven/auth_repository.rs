use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::auth::{Auth, NewAuth};
use crate::domain::types::identification::{IdentificationValue, NewIdentification};


pub enum RepoSelectError {
    NotFound,
    Unknown,
}

pub enum RepoCreateError {
    Unknown,
}

pub enum RepoUpdateError {
    NotFound,
    Unknown,
}

pub enum RepoDeleteError {
    NotFound,
    Unknown,
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
    async fn find_by_id(&self, conn: &T, user_id: Uuid) -> Result<Auth, String>;

    async fn find_by_identification(&self, conn: &T, identification: IdentificationValue) -> Result<Auth, String>;

    /// Insert the received entity in the persistence system
    async fn create(&self, conn: &T, auth: NewAuth) -> Result<Auth, String>;

    /// Update one single record already present in the persistence system
    async fn update_password(
        &self, 
        conn: &T, 
        user_id: Uuid, 
        new_hashed_password: String
    ) -> Result<Auth, String>;
   
    /// Update one single record already present in the persistence system
    async fn update_identifications(
        &self, 
        conn: &T, 
        identification_operation: UpdateIdentify<NewIdentification, Uuid>,
    ) -> Result<Auth, String>;

    /// Delete one single record from the persistence system
    async fn delete(&self, conn: &T, user_id: Uuid) -> Result<Auth, String>;
}