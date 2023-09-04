use async_trait::async_trait;

pub enum SmsSendError {
    Unknown,
    NotFound,
}

#[async_trait]
pub trait SmsServiceTrait<T> {
    async fn send_confirmation_sms(&self, conn: &T, phone_number: String, code: u32) -> Result<(), SmsSendError>;

    async fn send_reset_password_sms(&self, conn: &T, address: String, link: String) -> Result<(), SmsSendError>;
}