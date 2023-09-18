use async_trait::async_trait;

pub enum EmailSendError {
    Unknown,
    NotFound,
}

#[async_trait]
pub trait EmailServiceTrait<T> {
    async fn send_confirmation_email(&self, conn: &T, address: String, code: String) -> Result<(), EmailSendError>;

    async fn send_reset_password_email(&self, conn: &T, address: String, link: String) -> Result<(), EmailSendError>;
}