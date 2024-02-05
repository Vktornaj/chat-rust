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

pub struct Payload {
    pub token: String,
}

pub async fn execute<T>(
    secret: &[u8],
    cache_conn: &T,
    cache: &impl TokenCacheTrait<T>,
    payload: Payload,
) -> Result<SingleUseTokenData, Error> {
    let token_data = SingleUseTokenData::from_token(&payload.token, secret)
        .map_err(|err| match err {
            TokenDataError::Invalid(msg) => Error::Invalid(msg),
            TokenDataError::Expired(msg) => Error::Expired(msg),
            TokenDataError::Unknown(msg) => Error::Unknown(msg),
        })?;
    let token = cache
        .find_by_id(cache_conn, token_data.tkn_id)
        .await
        .map_err(|err| Error::Unknown(format!("{:?}", err)))?;
    cache.delete(cache_conn, token_data.tkn_id)
        .await
        .map_err(|err| Error::Unknown(format!("{:?}", err)))?;
    Ok(token)
}
