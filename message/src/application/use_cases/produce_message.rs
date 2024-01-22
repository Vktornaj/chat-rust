// use auth::domain::auth::Auth;
// use uuid::Uuid;

// use crate::application::port::driven::{
//     message_queue::MessageQueue, 
//     media_repository::Media
// };
// use common::domain::{
//     models::message::{Message, MessageContent as DomainMessageContent}, 
//     types::{text::Text, sender_type::Sender, recipient::Recipient, id::Id}
// };


// #[derive(Debug)]
// pub enum SendError {
//     InvalidData(String),
//     Unknown(String),
//     Conflict(String),
//     Unauthorized(String),
// }

// pub enum MessageContent {
//     Text(String),
//     Image(Vec<u8>),
//     Video(Vec<u8>),
//     Audio(Vec<u8>),
//     File(Vec<u8>),
// }

// impl TryFrom<MessageContent> for Media {
//     type Error = String;
//     fn try_from(content: MessageContent) -> Result<Self, String> {
//         match content {
//             MessageContent::Text(text) => Err("Text is not a media".to_string()),
//             MessageContent::Image(c) => Ok(Media::Image(c)),
//             MessageContent::Video(c) => Ok(Media::Video(c)),
//             MessageContent::Audio(c) => Ok(Media::Audio(c)),
//             MessageContent::File(c) => Ok(Media::File(c)),
//         }
//     }
// }

// pub struct Payload {
//     pub recipient: Uuid,
//     pub content: MessageContent,
// }

// pub async fn execute<T>(
//     conn: &T,
//     queue: &impl MessageQueue<T>,
//     secret: &[u8],
//     token: &String,
//     payload: Payload,
// ) -> Result<Uuid, SendError> {
//     // authenticate
//     let id = if let Ok(auth) = Auth::from_token(token, secret) {
//         auth.id
//     } else {
//         return Err(SendError::Unauthorized("Invalid token".to_string()));
//     };
//     // create message
//     let sender_contact_data = Id::try_from(id)
//         .map_err(|e| SendError::InvalidData(e.to_string()))?;
//     let recipient_contact_data = Id::try_from(payload.recipient)
//         .map_err(|e| SendError::InvalidData(e.to_string()))?;
//     // create message content
//     let content = match payload.content {
//         MessageContent::Text(text) => DomainMessageContent::Text(
//             Text::try_from(text).map_err(|e| SendError::InvalidData(e.to_string()))?,
//         ),
//         MessageContent::Image(binary) => DomainMessageContent::Image(binary),
//         MessageContent::Video(binary) => DomainMessageContent::Video(binary),
//         MessageContent::Audio(binary) => DomainMessageContent::Audio(binary),
//         MessageContent::File(binary) => DomainMessageContent::File(binary),
        
//     };
//     // create message
//     let message = Message::new(
//         Sender::User(sender_contact_data),
//         Recipient::User(recipient_contact_data),
//         content,
//     );
//     // add message to queue
//     queue
//         .add(conn, &message)
//         .await
//         .map_err(|e| SendError::Unknown(e.to_string()))?;
    
//     Ok(message.get_id().into())
// }

// #[cfg(test)]
// mod tests {}