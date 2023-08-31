use std::{sync::Mutex, collections::HashMap};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

// use super::{user_repository::{UserRepositoryTrait, NewUser, UpdateUser, FindUser}, errors};
use crate::application::port::driven::{
        user_cache::UserCacheTrait, 
        errors::{
            RepoSelectError, 
            RepoCreateError, 
            RepoDeleteError
        }
    };


pub struct InMemoryRepository();

#[async_trait]
impl UserCacheTrait<Mutex<HashMap<String, String>>> for InMemoryRepository {
    async fn find_by_id<T>(
        &self, 
        conn: &Mutex<HashMap<String, String>>, 
        id: String
    ) -> Result<Option<T>, RepoSelectError> 
    where 
        T: DeserializeOwned,
    {
        let lock = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(RepoSelectError::Unknown("Failed to lock mutex".to_string()))
        };
        match lock.get(&id) {
            Some(cache_user) => {
                let cache_user: T = serde_json::from_str(cache_user).unwrap();
                Ok(Some(cache_user))
            },
            None => Ok(None)
        }
    }

    async fn add_request<T>(
        &self, 
        conn: &Mutex<HashMap<String, String>>, 
        id: String, 
        item: T, 
        exp: u32
    ) -> Result<String, RepoCreateError> 
    where 
        T: Clone + Serialize + Send,
    {
        let mut lock = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(RepoCreateError::Unknown("Failed to lock mutex".to_string()))
        };
        match serde_json::to_string(&item) {
            Ok(item) => {
                lock.insert(id, item);
                Ok(id)
            },
            Err(_) => Err(RepoCreateError::Unknown("Failed to serialize item".to_string()))
        }
    }

    async fn delete(
        &self, 
        conn: &Mutex<HashMap<String, String>>, 
        id: String
    ) -> Result<(), RepoDeleteError> {
        let mut lock = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(RepoDeleteError::Unknown("Failed to lock mutex".to_string()))
        };
        match lock.remove(&id) {
            Some(cache_user) => Ok(()),
            None => Err(RepoDeleteError::NotFound)
        }
    }
}