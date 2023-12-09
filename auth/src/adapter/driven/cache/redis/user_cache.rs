use deadpool::managed::Pool;
use deadpool_redis::{Manager, Connection, redis::{cmd, RedisError}};
use async_trait::async_trait;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::to_string;

// use super::{user_repository::{UserRepositoryTrait, NewUser, UpdateUser, FindUser}, errors};
use crate::application::port::driven::auth_cache::AuthCacheTrait;


pub struct AuthCache();

#[async_trait]
impl AuthCacheTrait<Pool<Manager, Connection>> for AuthCache {
    async fn find_by_id<T>(&self, pool: &Pool<Manager, Connection>, id: String) -> Result<Option<T>, String>
    where
        T: DeserializeOwned,
    {
        let mut conn = pool.get().await.map_err(|e| {
            format!("Failed to get connection from pool: {}", e)
        })?;

        let result: Result<Option<String>, RedisError> = cmd("GET")
            .arg(&[id.to_string()])
            .query_async(&mut conn)
            .await;

        match result {
            Ok(data) => {
                if let Some(data) = data {
                    if let Ok(value) = serde_json::from_str(&data) {
                        Ok(Some(value))
                    } else {
                        Err("Failed to deserialize value".to_string())
                    }
                } else {
                    Ok(None)
                }
            }
            Err(err) => Err(format!("Failed to get value {}", err)),
        }
    }

    async fn add_request<T>(
        &self,
        pool: &Pool<Manager, Connection>,
        id: String,
        payload: T,
        exp: u32, // Expiration time in seconds
    ) -> Result<String, String>
    where
        T: Serialize + Send,
    {
        // Serialize payload to JSON
        let payload_str = to_string(&payload)
            .map_err(|e| format!("Serialization error: {}", e))?;
    
        let mut conn = pool.get().await.map_err(|e| {
            format!("Failed to get connection from pool: {}", e)
        })?;
    
        // Use SETEX to set the key with expiration
        let res: Result<(), RedisError> = cmd("SETEX")
            .arg(&[id.to_string(), exp.to_string(), payload_str])
            .query_async(&mut conn)
            .await;
    
        match res {
            Ok(_) => Ok(id),
            Err(e) => Err(format!("Failed to set value: {}", e)),
        }
    }

    async fn delete(&self, pool: &Pool<Manager, Connection>, id: String) -> Result<(), String> {
        let mut conn = pool.get().await.map_err(|e| {
            format!("Failed to get connection from pool: {}", e)
        })?;
    
        let result: Result<(), RedisError> = cmd("DEL")
            .arg(&[id.to_string()])
            .query_async(&mut conn)
            .await;
    
        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(format!("Failed to delete value {}", err))
        }
    }
}