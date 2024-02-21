use async_trait::async_trait;
use deadpool::managed::Pool;
use deadpool_redis::{redis::{cmd, RedisError}, Connection, Manager};
use serde_json::to_string;
use uuid::Uuid;

use crate::{application::port::driven::token_cache::{Error, SingleUseTokenData}, TokenCacheTrait};


pub struct TokenCache();

#[async_trait]
impl TokenCacheTrait<Pool<Manager, Connection>> for TokenCache {
    async fn find_by_id(&self, conn: &Pool<Manager, Connection>, id: Uuid) -> Result<SingleUseTokenData, Error> {
        let mut conn = conn.get().await.map_err(|e| {
            Error::Unknown(format!("Failed to get connection from pool: {}", e))
        })?;

        let result: Result<Option<String>, RedisError> = cmd("GET")
            .arg(&[id.to_string()])
            .query_async(&mut conn)
            .await;

        match result {
            Ok(data) => {
                if let Some(data) = data {
                    if let Ok(value) = serde_json::from_str(&data) {
                        Ok(value)
                    } else {
                        Err(Error::Unknown("Failed to deserialize value".to_string()))
                    }
                } else {
                    Err(Error::NotFound("Token not found".to_string()))
                }
            }
            Err(err) => Err(Error::Unknown(format!("Failed to get value {}", err))),
        }
    }

    async fn add(
        &self, 
        conn: &Pool<Manager, Connection>, 
        id: Uuid, 
        data: SingleUseTokenData,
        exp_sec: u32,
    ) -> Result<(), Error> {
        let data_str = to_string(&data)
            .map_err(|e| Error::Unknown(format!("Serialization error: {}", e)))?;
        let mut conn = conn.get().await.map_err(|e| {
            Error::Unknown(format!("Failed to get connection from pool: {}", e))
        })?;
        let res: Result<(), RedisError> = cmd("SETEX")
            .arg(&[id.to_string(), exp_sec.to_string(), data_str])
            .query_async(&mut conn)
            .await;
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Unknown(format!("Failed to set value {}", err))),
        }
    }
    
    async fn delete(&self, conn: &Pool<Manager, Connection>, id: Uuid) -> Result<(), Error> {
        let mut conn = conn.get().await.map_err(|e| {
            Error::Unknown(format!("Failed to get connection from pool: {}", e))
        })?;
        let res: Result<(), RedisError> = cmd("DEL")
            .arg(&[id.to_string()])
            .query_async(&mut conn)
            .await;
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::Unknown(format!("Failed to delete value {}", err))),
        }
    }
}