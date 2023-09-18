use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::domain::{
    types::{
        email::Email, 
        phone_number::PhoneNumber, 
        first_name::FirstName, 
        last_name::LastName, 
        birthday::Birthday, 
        nationality::Nationality, 
        language::Language, 
        code::Code, 
        error::ErrorMsg, 
        password::Password, 
        id::Id
    }, 
    user::NewUser};

use super::{errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
}, user_repository::UpdateUser};

#[derive(Clone, Deserialize, Serialize)]
pub struct CreateUserCache {
    pub email: Option<Email>,
    pub phone_number: Option<PhoneNumber>,
    pub hashed_password: String,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub birthday: Birthday,
    pub nationality: Nationality,
    pub languages: Vec<Language>,
    pub confirmation_code: Code,
}

impl CreateUserCache {
    pub fn new(
        email: Option<String>,
        phone_number: Option<String>,
        password: String,
        first_name: String,
        last_name: String,
        birthday: DateTime<Utc>,
        nationality: String,
        languages: Vec<String>,
        confirmation_code: Code,
    ) -> Result<Self, ErrorMsg> {
        if email.is_none() && phone_number.is_none() {
            return Err(ErrorMsg("email or phone number must be provided".to_string()));
        }
        if email.is_some() && phone_number.is_some() {
            return Err(ErrorMsg("email and phone number cannot be provided at the same time".to_string()));
        }
        let languages: Result<Vec<Language>, ErrorMsg> = languages.into_iter()
            .map(|x| Language::try_from(x))
            .collect();
        let hashed_password = match Password::try_from(password)?.hash_password() {
            Ok(hashed_password) => hashed_password,
            Err(e) => return Err(ErrorMsg(e.to_string())),
        };
        Ok(CreateUserCache { 
            email: email.map(|x| Email::try_from(x)).transpose()?, 
            phone_number: phone_number.map(|x| PhoneNumber::try_from(x)).transpose()?, 
            hashed_password, 
            first_name: FirstName::try_from(first_name)?, 
            last_name: LastName::try_from(last_name)?, 
            birthday: Birthday::try_from(birthday)?, 
            nationality: Nationality::try_from(nationality)?, 
            languages: languages?,
            confirmation_code
        })
    }

    pub fn to_new_user(self) -> NewUser {
        NewUser {
            email: self.email,
            phone_number: self.phone_number,
            hashed_password: self.hashed_password,
            first_name: self.first_name,
            last_name: self.last_name,
            birthday: self.birthday,
            nationality: self.nationality,
            languages: self.languages,
        }
    }
}

// update user contact data
#[derive(Clone, Deserialize, Serialize)]
pub struct UpdateUserCDCache {
    pub id: Id,
    pub email: Option<Option<Email>>,
    pub phone_number: Option<Option<PhoneNumber>>,
    pub confirmation_code: Code,
}

impl UpdateUserCDCache {
    pub fn to_update_user(self) -> UpdateUser {
        UpdateUser {
            id: self.id.into(),
            email: self.email,
            phone_number: self.phone_number,
            ..Default::default()
        }
    }
}


// recover password
#[derive(Clone, Deserialize, Serialize)]
pub struct RecoverPasswordCache {
    pub id: Id,
    pub hashed_new_password: String,
    pub confirmation_code: Code,
}

#[async_trait]
pub trait UserCacheTrait<T> {
    /// Find and return one single record from the persistence system by id
    async fn find_by_id<U>(&self, conn: &T, id: String) -> Result<Option<U>, RepoSelectError>
    where
        U: DeserializeOwned;

    /// Insert the received entity in the persistence system
    async fn add_request<U>(
        &self, 
        conn: &T, 
        transaction_id: String, 
        payload: U,
        exp: u32
    ) -> Result<String, RepoCreateError>
    where
        U: Clone + Serialize + Send;
    
    // /// Insert the received entity in the persistence system
    // async fn add_update_user(
    //     &self, 
    //     conn: &T, 
    //     transaction_id: String, 
    //     user: UpdateUserCDCache, 
    //     exp: u32
    // ) -> Result<String, RepoCreateError>;

    // /// Add request to recover password
    // async fn add_recover_password(
    //     &self, 
    //     conn: &T, 
    //     transaction_id: String, 
    //     user: RecoverPasswordCache, 
    //     exp: u32
    // ) -> Result<String, RepoCreateError>;

    /// Delete one single record from the persistence system
    async fn delete(&self, conn: &T, id: String) -> Result<(), RepoDeleteError>;
}