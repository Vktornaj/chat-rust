use uuid::Uuid;

use super::super::types::{
    sender_type::Sender, 
    recipient::Recipient, 
    // media_path::MediaPath,
    id::Id,
};


#[derive(Clone)]
pub struct Message {
    pub sender: Sender,
    pub recipient: Recipient,
    pub content: Vec<u8>,
    pub timestamp: u64,
    pub id: Id,
}

impl Message {
    pub fn new(sender: Sender, recipient: Recipient, content: Vec<u8>) -> Self {
        Self {
            sender,
            recipient,
            content,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            id: Id::try_from(Uuid::new_v4()).unwrap(),
        }
    }

    pub fn get_id(&self) -> Id {
        self.id.clone()
    }
}