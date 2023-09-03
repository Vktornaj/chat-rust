use async_trait::async_trait;

pub enum EmailSendError {
    Unknown,
    NotFound,
}

#[async_trait]
pub trait EmailServiceTrait<T> {
    async fn send_confirmation_email(&self, conn: &T, address: String, code: u32) -> Result<(), EmailSendError>;
}