use async_trait::async_trait;

pub enum SmsSendError {
    Unknown,
    NotFound,
}

#[async_trait]
pub trait SmsServiceTrait<T> {
    /// Find and return one single record from the persistence system by id
    async fn send_confirmation_sms(&self, conn: &T, phone_number: String, code: u32) -> Result<(), SmsSendError>;
}