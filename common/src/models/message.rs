use uuid::Uuid;
use serde::{Deserialize, Serialize};

use super::super::types::{
    text::Text,
    sender_type::Sender, 
    recipient::Recipient, 
    // media_path::MediaPath,
    id::Id,
};


#[derive(Clone, Deserialize, Serialize, Debug)]
pub enum MessageContent {
    Text(Text),
    Image(Vec<u8>),
    Video(Vec<u8>),
    Audio(Vec<u8>),
    File(Vec<u8>),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum MessageStatus {
    Sent,
    Delivered,
    Read,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Message {
    pub sender: Sender,
    pub recipient: Recipient,
    pub content: MessageContent,
    pub timestamp: u64,
    pub status: MessageStatus,
    pub id: Id,
}

impl Message {
    pub fn new(sender: Sender, recipient: Recipient, content: MessageContent) -> Self {
        Self {
            sender,
            recipient,
            content,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            status: MessageStatus::Sent,
            id: Id::try_from(Uuid::new_v4()).unwrap(),
        }
    }

    pub fn get_id(&self) -> Id {
        self.id.clone()
    }
}