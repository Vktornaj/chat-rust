use user::types::id::Id;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use super::types::{
    text::Text,
    sender::Sender, 
    recipient::Recipient, media_path::MediaPath,
};


#[derive(Clone, Deserialize, Serialize)]
pub enum MessageContent {
    Text(Text),
    Image(MediaPath),
    Video(MediaPath),
    Audio(MediaPath),
    File(MediaPath),
}

#[derive(Clone, Deserialize, Serialize)]
pub enum MessageStatus {
    Sent,
    Delivered,
    Read,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Message {
    sender: Sender,
    recipient: Recipient,
    content: MessageContent,
    timestamp: u64,
    status: MessageStatus,
    id: Id,
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