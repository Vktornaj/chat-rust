use async_trait::async_trait;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

use common::domain::types::{error::ErrorMsg, id::Id};
use crate::domain::types::{
    code::Code, 
    password::Password,
    identification::IdentificationValue,
};


#[derive(Clone, Deserialize, Serialize)]
pub struct CreateAuthRequest {
    pub hashed_password: String,
    pub confirmation_code: Code,
    pub identity: IdentificationValue,
}

impl CreateAuthRequest {
    pub fn new(
        password: String,
        confirmation_code: Code,
        identity: IdentificationValue,
    ) -> Result<Self, ErrorMsg> {
        let hashed_password = match Password::try_from(password)?.hash_password() {
            Ok(hashed_password) => hashed_password,
            Err(e) => return Err(ErrorMsg(e.to_string())),
        };
        Ok(CreateAuthRequest { 
            hashed_password, 
            confirmation_code,
            identity,
        })
    }
}

// update user contact data
#[derive(Clone, Deserialize, Serialize)]
pub struct AddIdentificationRequest {
    pub user_id: Id,
    pub identity: IdentificationValue,
    pub confirmation_code: Code,
}

// recover password
#[derive(Clone, Deserialize, Serialize)]
pub struct RecoverPasswordCache {
    pub user_id: Id,
    pub hashed_new_password: String,
    pub confirmation_code: Code,
}

#[async_trait]
pub trait AuthCacheTrait<T> {
    /// Find and return one single record from the persistence system by id
    async fn find_by_id<U>(&self, conn: &T, id: String) -> Result<Option<U>, String>
    where
        U: DeserializeOwned;

    /// Insert the received entity in the persistence system
    async fn add_request<U>(
        &self, 
        conn: &T, 
        transaction_id: String, 
        payload: U,
        exp: u32
    ) -> Result<String, String>
    where
        U: Clone + Serialize + Send;
    
    /// Delete one single record from the persistence system
    async fn delete(&self, conn: &T, id: String) -> Result<(), String>;
}