use async_trait::async_trait;
use chrono::NaiveDate;
use uuid::Uuid;

use super::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
    RepoUpdateError
};

use crate::domain::{
    user::{User, NewUser}, 
    types::{
        language::Language, 
        nationality::Nationality, 
        first_name::FirstName, 
        last_name::LastName, 
        birthday::Birthday
    }
};

pub struct DateRange(pub Option<NaiveDate>, pub Option<NaiveDate>);

#[derive(Default)]
pub struct FindUser {
    pub birthday: Option<DateRange>,
    pub languages: Option<Vec<Language>>,
    pub nationality: Option<Nationality>,
    pub created_at: Option<DateRange>,
}

#[derive(Default)]
pub struct UpdateUser {
    pub id: Uuid,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub birthday: Option<Birthday>,    
    pub nationality: Option<Nationality>,
    pub languages: Option<Vec<Language>>,
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
        find_user: FindUser,
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