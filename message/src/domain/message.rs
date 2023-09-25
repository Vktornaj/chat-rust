use user::types::id::Id;
use uuid::Uuid;

use super::types::{
    file::File,
    audio::Audio,
    video::Video,
    text::Text,
    image::Image, 
    sender::Sender, 
    recipient::Recipient, media_path::MediaPath,
};


pub enum MessageContent {
    Text(Text),
    Image(MediaPath),
    Video(MediaPath),
    Audio(MediaPath),
    File(MediaPath),
}

pub enum MessageStatus {
    Sent,
    Delivered,
    Read,
}

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