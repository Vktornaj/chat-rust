use chrono::{DateTime, Utc};
use common::domain::types::{recipient::Recipient, sender_type::Sender};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(PartialEq, Serialize, Deserialize)]
pub enum MessageType {
    Text,
    Image,
    Video,
    Audio,
    File,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub sender: Sender,
    pub recipient: Recipient,
    pub message_type: MessageType,
    pub content: Vec<u8>,
    pub deleted: bool,
    pub received_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub sender: Sender,
    pub recipient: Recipient,
    pub message_type: MessageType,
    pub content: Vec<u8>,
}