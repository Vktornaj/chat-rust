use async_trait::async_trait;

use common::domain::models::message::Message;
use super::errors::QueueAddError;


#[async_trait]
pub trait MessageQueue<T> {
    async fn add(&self, conn: &T, message: &Message) -> Result<(), QueueAddError>;
}