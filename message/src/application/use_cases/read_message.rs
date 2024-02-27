use chrono::Utc;
use common::domain::types::id::Id;
use uuid::Uuid;

use crate::application::port::driven::message_repository::{MessageRepositoryTrait, UpdateMessage};


pub enum Error {
    NotFound(String),
    DatabaseError(String),
    ConnectionError(String),
    Unauthorized(String),
}

pub struct Payload {
    pub message_id: Uuid,
    pub user_id: Id,
}

pub async fn execute<T, U>(
    conn: &T,
    message_repository: &impl MessageRepositoryTrait<T>,
    payload: Payload,
) -> Result<Uuid, Error> {
    let message = match message_repository.find_by_id(conn, payload.message_id).await {
        Ok(message) => message,
        Err(err) => return Err(Error::DatabaseError(err.to_string())),
    };
    if message.recipient != payload.user_id {
        return Err(Error::Unauthorized("User is not the recipient of the message".to_string()));
    }
    let update_message = UpdateMessage {
        id: message.id,
        read_at: Some(Some(Utc::now())),
        received_at: None,
    };
    match message_repository.update(conn, &update_message).await {
        Ok(message) => Ok(message.id),
        Err(err) => Err(Error::DatabaseError(err.to_string())),
    }
}
