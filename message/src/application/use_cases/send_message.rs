use uuid::Uuid;

use crate::{
    application::port::driven::{media_repository::{Media, MediaRepository}, message_repository::MessageRepository}, 
    domain::message::{MessageType, NewMessage}
};


pub enum Error {
    NotFound(String),
    DatabaseError(String),
    ConnectionError(String),
}

pub struct Payload {
    pub new_message: NewMessage,
}

pub async fn execute<T, U>(
    conn: &T,
    message_repository: &impl MessageRepository<T>,
    conn_media: &U,
    media_repository: &impl MediaRepository<U>,
    payload: Payload,
) -> Result<Uuid, Error> {
    let new_message = match payload.new_message.message_type {
        MessageType::Text => NewMessage {
            sender: payload.new_message.sender,
            recipient: payload.new_message.recipient,
            message_type: MessageType::Text,
            content: payload.new_message.content,
        },
        _ => {
            let media = match payload.new_message.message_type {
                MessageType::Image => Media::Image(payload.new_message.content),
                MessageType::Video => Media::Video(payload.new_message.content),
                MessageType::Audio => Media::Audio(payload.new_message.content),
                MessageType::File => Media::File(payload.new_message.content),
                _ => panic!("Invalid message type"),
            };
            let media_url = media_repository.add(conn_media, &media).await;
            let media_url = match media_url {
                Ok(media_url) => media_url,
                Err(err) => return Err(Error::ConnectionError(err.to_string())),
            };
            NewMessage {
                sender: payload.new_message.sender,
                recipient: payload.new_message.recipient,
                message_type: payload.new_message.message_type,
                content: media_url.as_bytes().to_vec(),
            }
        }
    };
    match message_repository.create(conn, &new_message).await {
        Ok(message) => Ok(message.id),
        Err(err) => Err(Error::DatabaseError(err.to_string())),
    }
}
