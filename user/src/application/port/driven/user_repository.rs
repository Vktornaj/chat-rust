use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
    RepoUpdateError
};

use crate::domain::user::{User, Email, PhoneNumber, Password, FirstName, LastName, Birthday, Nationality, Language};


pub struct DateRange(pub Option<DateTime<Utc>>, pub Option<DateTime<Utc>>);

pub struct NewUser {
    pub email: Option<Email>,
    pub phone_number: Option<PhoneNumber>,
    pub password: Option<Password>,
    pub hashed_password: Option<String>,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub birthday: Birthday,
    pub nationality: Nationality,
    pub languages: Vec<Language>,
}

pub struct FindUser {
    pub email: Option<Email>,
    pub phone_number: Option<PhoneNumber>,
    pub birthday: Option<DateRange>,
    pub languages: Option<Vec<Language>>,
    pub nationality: Option<Nationality>,
    pub created_at: Option<DateRange>,
}

#[derive(Default)]
pub struct UpdateUser {
    pub id: Uuid,
    pub email: Option<Option<Email>>,
    pub phone_number: Option<Option<PhoneNumber>>,
    pub hashed_password: Option<Option<String>>,
    pub first_name: Option<Option<FirstName>>,
    pub last_name: Option<Option<LastName>>,
    pub birthday: Option<Option<Birthday>>,    
    pub nationality: Option<Option<Nationality>>,
    pub languages: Option<Option<Vec<Language>>>,
}

// impl Default for UpdateUser {
//     fn default() -> Self { 
//         UpdateUser { 
//             id: (), 
//             email: (), 
//             phone_number: (), 
//             hashed_password: (), 
//             first_name: (), 
//             last_name: (), 
//             birthday: (), 
//             nationality: (), 
//             languages: () 
//         }
//     }
// }

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