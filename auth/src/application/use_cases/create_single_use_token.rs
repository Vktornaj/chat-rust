use crate::{
    application::port::driven::token_cache::TokenCacheTrait, 
    domain::types::single_use_token::SingleUseTokenData, TokenData,
};


const DURATION: i64 = 600;

#[derive(Debug)]
pub enum Error {
    Unauthorized,
    Unknown(String),   
}

pub struct Payload {
    pub duration: i64,
}

impl Default for Payload {
    fn default() -> Self {
        Payload { duration: DURATION }
    }
}

pub async fn execute<T>(
    secret: &[u8], 
    cache_conn: &T,
    cache: &impl TokenCacheTrait<T>,
    token: String,
    payload: Payload,
) -> Result<String, Error> {

    let user_token_data = TokenData::from_token(&token, secret)
        .map_err(|_| Error::Unauthorized)?;

    let single_use_token_data = SingleUseTokenData::new(
        &user_token_data.id.try_into().unwrap(), 
        payload.duration
    );

    if let Err(err) = cache.add(
        cache_conn, 
        single_use_token_data.tkn_id, 
        single_use_token_data.clone(), 
        payload.duration as u32
    ).await {
        return Err(Error::Unknown(format!("{:?}", err)));
    }
    Ok(single_use_token_data.token(secret))
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::adapter::driven::cache::redis::token_cache::TokenCache;
    use crate::create_single_use_token::Payload;
    use common::adapter::{
        cache::create_pool,
        config::Config,
    };

    #[tokio::test]
    pub async fn test_create_single_use_token() {
        let pool = create_pool().await;
        let config = Config::new();
        let cache = TokenCache();
        let secret = &config.secret;
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MTIzNjc3NTksImlkIjoiZjg3NzIyOTctYjA2Yy00OGU1LTk2MDItOTJjOTAxM2YwMGM3IiwidGtuX2lkIjoiYzNiNjQ2MjctZWQwYS00NDdiLTg0ODEtNjYwNjBhOWFhNzg4In0.qWdatXm7-i9SHe2jkX-y2NEZLWbkfOtxJtwWiCiTbPo";
        
        let result = execute(secret, &pool, &cache, token.to_string(), Payload { duration: 600 }).await;
        println!("{:?}", result);
        assert!(result.is_ok());

        let result = execute(secret, &pool, &cache, "".to_string(), Payload { duration: 600 }).await;
        assert!(result.is_err());
    }
}
