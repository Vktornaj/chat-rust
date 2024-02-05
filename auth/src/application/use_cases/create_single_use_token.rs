use common::domain::types::id::Id;
use futures::TryFutureExt;

use crate::{
    application::port::driven::token_cache::TokenCacheTrait, 
    domain::types::single_use_token::SingleUseTokenData,
};

#[derive(Debug)]
pub enum Error {
    Invalid(String),
    Unknown(String),   
}

pub struct Payload {
    pub user_id: Id,
    pub duration: i64,
}

pub async fn execute<T, U>(
    secret: &[u8], 
    cache_conn: &U,
    cache: &impl TokenCacheTrait<U>,
    payload: Payload,
) -> Result<String, Error> {
    let token_data = SingleUseTokenData::new(&payload.user_id, payload.duration);
    cache.add(
        cache_conn, 
        token_data.tkn_id.clone(), 
        token_data.clone(), 
        payload.duration as u32 * 60,
    ).map_err(|err| Error::Unknown(format!("{:?}", err))).await?;
    Ok(token_data.token(secret))
}
