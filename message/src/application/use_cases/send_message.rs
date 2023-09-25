use auth::domain::auth::Auth;
use user::types::id::Id;
use uuid::Uuid;

use crate::{
    application::port::driven::{message_queue::MessageQueue, media_repository::{self, MediaRepository, Media}},
    domain::{
        message::{Message, MessageContent as DomainMessageContent, self},
        types::{
            audio::Audio,
            file::File,
            image::Image,
            recipient::Recipient,
            sender::Sender,
            text::Text,
            user_contact_data::UserContactData,
            video::Video, media_path::MediaPath,
        },
    },
};

#[derive(Debug)]
pub enum SendError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    Unautorized(String),
}

pub enum MessageContent {
    Text(String),
    Image(Vec<u8>),
    Video(Vec<u8>),
    Audio(Vec<u8>),
    File(Vec<u8>),
}

impl TryFrom<MessageContent> for Media {
    type Error = String;
    fn try_from(content: MessageContent) -> Result<Self, String> {
        match content {
            MessageContent::Text(text) => Err("Text is not a media".to_string()),
            MessageContent::Image(c) => Ok(Media::Image(c)),
            MessageContent::Video(c) => Ok(Media::Video(c)),
            MessageContent::Audio(c) => Ok(Media::Audio(c)),
            MessageContent::File(c) => Ok(Media::File(c)),
        }
    }
}

pub struct Payload {
    pub recipient: Uuid,
    pub content: MessageContent,
}

pub async fn execute<T>(
    conn: &T,
    queue: &impl MessageQueue<T>,
    media_repository: &impl MediaRepository<T>,
    secret: &[u8],
    token: &String,
    payload: Payload,
) -> Result<Uuid, SendError> {
    // authenticate
    let id = if let Ok(auth) = Auth::from_token(token, secret) {
        auth.id
    } else {
        return Err(SendError::Unautorized("Invalid token".to_string()));
    };
    // create message
    let sender_contact_data = Id::try_from(id)
        .map_err(|e| SendError::InvalidData(e.to_string()))?;
    let recipient_contact_data = Id::try_from(payload.recipient)
        .map_err(|e| SendError::InvalidData(e.to_string()))?;
    // create message content
    let content = match payload.content {
        MessageContent::Text(text) => DomainMessageContent::Text(
            Text::try_from(text).map_err(|e| SendError::InvalidData(e.to_string()))?,
        ),
        msg_c => {
            let content = Media::try_from(msg_c)
                .map_err(|e| SendError::InvalidData(e.to_string()))?;
            // add media to repository
            let path = media_repository
                .add(conn, &content)
                .await
                .map_err(|e| SendError::Unknown(e.to_string()))?;
            let path = MediaPath::try_from(path)
                .map_err(|e| SendError::Unknown(e.to_string()))?;
            match content {
                Media::Image(_) => DomainMessageContent::Image(path),
                Media::Video(_) => DomainMessageContent::Video(path),
                Media::Audio(_) => DomainMessageContent::Audio(path),
                Media::File(_) => DomainMessageContent::File(path),
            }
        }
    };
    // create message
    let message = Message::new(
        Sender::User(sender_contact_data),
        Recipient::User(recipient_contact_data),
        content,
    );
    // add message to queue
    queue
        .add(conn, &message)
        .await
        .map_err(|e| SendError::Unknown(e.to_string()))?;
    
    Ok(message.get_id().into())
}

#[cfg(test)]
mod tests {}