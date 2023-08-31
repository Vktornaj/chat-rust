use deadpool_redis::redis::{FromRedisValue, self};
use rocket::serde::json::serde_json;
use crate::domain::user::CacheUser;


impl FromRedisValue for CacheUser {
    fn from_redis_value(value: &redis::Value) -> Result<Self, redis::RedisError> {
        match value {
            redis::Value::Data(data) => {
                let user: CacheUser = serde_json::from_slice(data).unwrap();
                Ok(user)
            },
            _ => Err(redis::RedisError::from((
                redis::ErrorKind::TypeError,
                "value is not a string or an integer",
            ))),
        }
    }
}