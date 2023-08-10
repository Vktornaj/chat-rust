use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
    RepoUpdateError
};

use crate::domain::user::User;


pub struct DateRange(pub Option<DateTime<Utc>>, pub Option<DateTime<Utc>>);

pub struct NewUser {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub hashed_password: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub birthday: DateTime<Utc>,
    pub nationality: String,
    pub languages: Vec<String>,
}

impl NewUser {
    
}

pub struct FindUser {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub birthday: Option<DateRange>,
    pub nationality: Option<String>,
    pub languages: Option<Vec<String>>,
    pub created_at: Option<DateRange>,
}

pub struct UpdateUser {
    pub id: Uuid,
    pub email: Option<Option<String>>,
    pub phone_number: Option<Option<String>>,
    pub hashed_password: Option<Option<String>>,
    pub first_name: Option<Option<String>>,
    pub last_name: Option<Option<String>>,
    pub birthday: Option<Option<DateTime<Utc>>>,    
    pub nationality: Option<Option<String>>,
    pub languages: Option<Option<Vec<String>>>,
}

// TODO: improve criteria
#[async_trait]
pub trait UserRepositoryTrait<T> {
    /// Find and return one single record from the persistence system by id
    async fn find_by_id(&self, conn: &T, id: Uuid) -> Result<User, RepoSelectError>;
    
    /// Find and return some records from the persistence system by criteria
    async fn find_by_criteria(
        &self, 
        conn: &T, 
        find_user: &FindUser,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<User>, RepoSelectError>;

    /// Insert the received entity in the persistence system
    async fn create(&self, conn: &T, user: NewUser) -> Result<User, RepoCreateError>;

    /// Update one single record already present in the persistence system
    async fn update(&self, conn: &T, user: UpdateUser) -> Result<User, RepoUpdateError>;

    /// Delete one single record from the persistence system
    async fn delete(&self, conn: &T, id: Uuid) -> Result<User, RepoDeleteError>;
}