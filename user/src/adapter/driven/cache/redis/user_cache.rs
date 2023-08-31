use deadpool::managed::Pool;
use deadpool_redis::{Manager, Connection, redis::{cmd, FromRedisValue}};
use async_trait::async_trait;
use rocket::serde::json::serde_json;
use uuid::Uuid;

// use super::{user_repository::{UserRepositoryTrait, NewUser, UpdateUser, FindUser}, errors};
use crate::{
    domain::{user::CacheUser, types::{email::Email, phone_number::PhoneNumber}}, 
    application::port::driven::{user_cache::UserCacheTrait, errors::{RepoSelectError, RepoCreateError, RepoDeleteError}}
};


pub struct RedisCache();

#[async_trait]
impl UserCacheTrait<Pool<Manager, Connection>> for RedisCache {
    async fn find_by_id<E>(&self, pool: &Pool<Manager, Connection>, id: String) -> Result<E, RepoSelectError> {
        let mut conn = pool.get().await.unwrap();
        let value = cmd("GET")
            .arg(&[id.to_string()])
            .query_async(&mut conn)
            .await.unwrap();
        Ok(value)
    }
    
    async fn create(
        &self, 
        pool: &Pool<Manager, Connection>, 
        id: String, 
        cache_user: CacheUser, 
        exp: u32
    ) -> Result<CacheUser, RepoCreateError> {
        let mut conn = pool.get().await.unwrap();
        let cache_user = serde_json::to_string(&cache_user).unwrap();
        let value = cmd("SET")
            .arg(&[id.to_string(), cache_user])
            .query_async(&mut conn)
            .await.unwrap();
        Ok(value)
    }

    async fn delete(&self, conn: &Pool<Manager, Connection>, id: String) -> Result<CacheUser, RepoDeleteError> {
        let mut conn = conn.get().await.unwrap();
        let value = cmd("DEL")
            .arg(&[id])
            .query_async(&mut conn)
            .await.unwrap();
        Ok(value)
    }

}