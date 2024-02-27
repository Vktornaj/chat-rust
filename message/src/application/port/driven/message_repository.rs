use async_trait::async_trait;

use common::domain::types::{recipient::Recipient, sender_type::Sender};
use uuid::Uuid;
use crate::domain::message::{Message, NewMessage};


pub enum Error {
    NotFound(String),
    DatabaseError(String),
    ConnectionError(String),
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::NotFound(err) => format!("Not found: {}", err),
            Error::DatabaseError(err) => format!("Database error: {}", err),
            Error::ConnectionError(err) => format!("Connection error: {}", err),
        }
    }
}

pub struct UpdateMessage {
    pub id: Uuid,
    pub read_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
    pub received_at: Option<Option<chrono::DateTime<chrono::Utc>>>,
}

#[async_trait]
pub trait MessageRepositoryTrait<T> {
    async fn create(&self, conn: &T, new_message: NewMessage) -> Result<Message, Error>;
    async fn find_list(
        &self, 
        conn: &T, 
        sender: Sender,
        recipient: Recipient,
        limit: i64, 
        offset: Option<u64>, 
        ascending: bool,
    ) -> Result<Vec<Message>, Error>;
    async fn find_by_id(&self, conn: &T, id: Uuid) -> Result<Message, Error>;
    async fn update(&self, conn: &T, message: &UpdateMessage) -> Result<Message, Error>;
    async fn delete(&self, conn: &T, id: Uuid) -> Result<(), Error>;
}
