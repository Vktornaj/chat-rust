use async_trait::async_trait;
use uuid::Uuid;

pub use crate::domain::types::single_use_token::SingleUseTokenData;


#[derive(Debug)]
pub enum Error {
    NotFound(String),
    Unknown(String),
}

#[async_trait]
pub trait TokenCacheTrait<T> {
    async fn add(
        &self, 
        conn: &T, 
        id: Uuid, 
        data: SingleUseTokenData,
        exp: u32,
    ) -> Result<(), Error>;
    async fn find_by_id(
        &self, 
        conn: &T, 
        id: Uuid
    ) -> Result<SingleUseTokenData, Error>;
    async fn delete(&self, conn: &T, id: Uuid) -> Result<(), Error>;
}
