use async_trait::async_trait;


use super::errors::MediaError;


#[derive(Debug)]
pub enum Media {
    Image(Vec<u8>),
    Video(Vec<u8>),
    Audio(Vec<u8>),
    File(Vec<u8>),
}

#[async_trait]
pub trait MediaRepository<T> {
    async fn add(&self, conn: &T, media: &Media) -> Result<String, MediaError>;   
}