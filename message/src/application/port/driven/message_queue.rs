use async_trait::async_trait;


use super::errors::QueueAddError;
use crate::domain::message::Message;


#[async_trait]
pub trait MessageQueue<T> {
    async fn add(&self, conn: &T, message: &Message) -> Result<(), QueueAddError>;
}