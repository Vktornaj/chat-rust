use crate::{
    application::port::driven::token_cache::TokenCacheTrait, 
    domain::types::single_use_token::{SingleUseTokenData, TokenDataError},
};

#[derive(Debug)]
pub enum Error {
    Invalid(String),
    Expired(String),
    Unknown(String),
}

pub async fn execute<T>(
    secret: &[u8],
    cache_conn: &T,
    cache: &impl TokenCacheTrait<T>,
    single_use_token: String,
) -> Result<SingleUseTokenData, Error> {
    let single_use_token_data = SingleUseTokenData::from_token(&single_use_token, secret)
        .map_err(|err| match err {
            TokenDataError::Invalid(msg) => Error::Invalid(msg),
            TokenDataError::Expired(msg) => Error::Expired(msg),
            TokenDataError::Unknown(msg) => Error::Unknown(msg),
        })?;
    cache
        .find_by_id(cache_conn, single_use_token_data.tkn_id)
        .await
        .map_err(|err| Error::Unknown(format!("{:?}", err)))?;
    cache.delete(cache_conn, single_use_token_data.tkn_id)
        .await
        .map_err(|err| Error::Unknown(format!("{:?}", err)))?;
    Ok(single_use_token_data)
}

#[cfg(test)]
mod test {
    use super::execute;
    use crate::TokenCache;
    use common::adapter::cache::create_pool;
    use common::adapter::config::Config;

    #[tokio::test]
    pub async fn test_create_single_use_token() {
        let pool = create_pool().await;
        let config = Config::new();
        let cache = TokenCache {};
        let secret = &config.secret;
        let token = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJleHAiOjE3MDcyMjIwMTYsInVzZXJfaWQiOiJmODc3MjI5Ny1iMDZjLTQ4ZTUtOTYwMi05MmM5MDEzZjAwYzciLCJ0a25faWQiOiI1YWZjYjlkMS1mNTVhLTQxZTItYTMyZi00OTk1MzBmNzg3NTAifQ.NyCozv5b5uSDWVhSWchJwqkPlEsgNwkkkKPkgTHL7Kg";
        let result = execute(secret, &pool, &cache, token.to_string()).await;
        assert!(result.is_ok());
        let result = execute(secret, &pool, &cache, token.to_string()).await;
        assert!(result.is_err());
    }
}
