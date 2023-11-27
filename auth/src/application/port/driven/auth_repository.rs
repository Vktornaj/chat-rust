use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::auth::Auth;


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

pub struct UpdateAuth {
    pub id: Uuid,
    pub new_hashed_password: Option<String>,
    pub token_metadata: Option<UpdateTokenMetadata>,
}

pub struct UpdateTokenMetadata {
    pub id: Uuid,
    pub last_use_timestamp: Option<i64>,
    pub is_active: Option<bool>,
}

pub struct FindContactInfo {
    pub email: Option<String>,
    pub phone_number: Option<String>,
}

#[async_trait]
pub trait AuthRepositoryTrait<T> {
    /// Find and return one single record from the persistence system by id
    async fn find_by_id(&self, conn: &T, id: Uuid) -> Result<Auth, RepoSelectError>;

    async fn find_by_contact_info(
        &self, 
        conn: &T, 
        contact_info: FindContactInfo,
    ) -> Result<Auth, RepoSelectError>;

    /// Insert the received entity in the persistence system
    async fn create(&self, conn: &T, auth: Auth) -> Result<Auth, RepoCreateError>;

    /// Update one single record already present in the persistence system
    async fn update(
        &self, 
        conn: &T, 
        id: Uuid, 
        new_hashed_password: String
    ) -> Result<Auth, RepoUpdateError>;

    /// Delete one single record from the persistence system
    async fn delete(&self, conn: &T, id: Uuid) -> Result<Auth, RepoDeleteError>;
}