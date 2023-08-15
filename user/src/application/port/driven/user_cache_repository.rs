use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::user::CacheUser;

use super::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
};

#[async_trait]
pub trait UserRepositoryTrait<T> {
    /// Find and return one single record from the persistence system by id
    async fn find_by_id(&self, conn: &T, id: Uuid) -> Result<CacheUser, RepoSelectError>;
    
    /// Insert the received entity in the persistence system
    async fn create(&self, conn: &T, user: CacheUser) -> Result<CacheUser, RepoCreateError>;

    /// Delete one single record from the persistence system
    async fn delete(&self, conn: &T, id: Uuid) -> Result<CacheUser, RepoDeleteError>;
}