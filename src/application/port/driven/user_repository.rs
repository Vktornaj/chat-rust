use async_trait::async_trait;

use super::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
    RepoUpdateError
};

use crate::domain::user::User;


#[async_trait]
pub trait UserRepository<T> {
    /// Find and return one single record from the persistence system    
    async fn find_one(&self, conn: &T, username: &String) -> Result<User, RepoSelectError>;

    /// Insert the received entity in the persistence system
    async fn create(&self, conn: &T, user: User) -> Result<User, RepoCreateError>;

    /// Update one single record already present in the persistence system
    async fn update(&self, conn: &T, user: User) -> Result<User, RepoUpdateError>;

    /// Delete one single record from the persistence system
    async fn delete(&self, conn: &T, username: &String) -> Result<User, RepoDeleteError>;
}