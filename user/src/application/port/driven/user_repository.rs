use async_trait::async_trait;
use chrono::{DateTime, Utc};

use super::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
    RepoUpdateError
};

use crate::domain::user::User;


pub struct FindUser {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub nationality: Option<String>,
    pub languages: Option<Vec<String>>,
    pub created_at: Option<String>
}

pub struct UpdateUser {
    pub id: i32,
    pub email: Option<Option<String>>,
    pub phone_number: Option<Option<String>>,
    pub password: Option<Option<String>>,
    pub first_name: Option<Option<String>>,
    pub last_name: Option<Option<String>>,
    pub birthday: Option<Option<DateTime<Utc>>>,    
    pub nationality: Option<Option<String>>,
    pub languages: Option<Option<Vec<String>>>,
}

#[async_trait]
pub trait UserRepositoryTrait<T> {
    /// Find and return one single record from the persistence system    
    async fn find_by_id(
        &self, 
        conn: &T, 
        id: i32
    ) -> Result<User, RepoSelectError>;
    
    async fn find_one_by_email(
        &self, 
        conn: &T, 
        email: &String
    ) -> Result<User, RepoSelectError>;
   
    async fn find_one_by_phone_number(
        &self, 
        conn: &T, 
        phone_number: &String
    ) -> Result<User, RepoSelectError>;

    /// Insert the received entity in the persistence system
    async fn create(&self, conn: &T, user: User) -> Result<User, RepoCreateError>;

    /// Update one single record already present in the persistence system
    async fn update(&self, conn: &T, user: UpdateUser) -> Result<User, RepoUpdateError>;

    /// Delete one single record from the persistence system
    async fn delete(&self, conn: &T, id: i32) -> Result<User, RepoDeleteError>;
}