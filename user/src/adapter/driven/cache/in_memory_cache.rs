use std::{sync::Mutex, collections::HashMap};
use async_trait::async_trait;

// use super::{user_repository::{UserRepositoryTrait, NewUser, UpdateUser, FindUser}, errors};
use crate::{
    domain::user::CacheUser, 
    application::port::driven::{
        user_cache::UserCacheTrait, 
        errors::{
            RepoSelectError, 
            RepoCreateError, 
            RepoDeleteError
        }
    }
};


pub struct InMemoryRepository();

#[async_trait]
impl UserCacheTrait<Mutex<HashMap<String, CacheUser>>> for InMemoryRepository {
    async fn find_by_id(&self, conn: &Mutex<HashMap<String, CacheUser>>, id: String) -> Result<CacheUser, RepoSelectError> {
        let lock = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(RepoSelectError::Unknown("Failed to lock mutex".to_string()))
        };
        let res = lock.get(&id);
        if res.is_none() {
            return Err(RepoSelectError::NotFound)
        }
        Ok(res.unwrap().clone())
    }

    async fn create(
        &self, 
        conn: &Mutex<HashMap<String, CacheUser>>, 
        id: String, 
        user: CacheUser, 
        exp: u32
    ) -> Result<CacheUser, RepoCreateError> {
        let mut lock = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(RepoCreateError::Unknown("Failed to lock mutex".to_string()))
        };
        lock.insert(id, user.clone());
        println!("exp: {}", exp);
        Ok(user)
    }

    async fn delete(&self, conn: &Mutex<HashMap<String, CacheUser>>, id: String) -> Result<CacheUser, RepoDeleteError> {
        let mut lock = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(RepoDeleteError::Unknown("Failed to lock mutex".to_string()))
        };
        match lock.remove(&id) {
            Some(cache_user) => Ok(cache_user),
            None => Err(RepoDeleteError::NotFound)
        }
    }
}