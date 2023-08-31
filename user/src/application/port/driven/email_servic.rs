use async_trait::async_trait;

pub enum EmailSendError {
    Unknown,
    NotFound,
}

#[async_trait]
pub trait UserCacheRepositoryTrait<T> {
    /// Find and return one single record from the persistence system by id
    async fn send_confirmation_email(&self, conn: &T, address: String, code: u32) -> Result<(), EmailSendError>;
}