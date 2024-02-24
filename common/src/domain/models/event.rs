use std::{sync::Arc, collections::VecDeque};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::types::{recipient::Recipient, sender_type::Sender};


// TODO: Build an adapter
pub type EventQueue = Arc<RwLock<VecDeque<Event>>>;

#[derive(Clone, Serialize, Deserialize)]
pub enum EventOwner {
    Sender(Sender),
    Recipient(Recipient),
}

pub struct Event {
    pub id: Uuid,
    pub owner: EventOwner,
    pub content: Vec<u8>,
    pub timestamp: u64,
}

impl Event {
    pub fn new(owner: EventOwner, content: Vec<u8>) -> Self {
        Self {
            owner,
            content,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            id: Uuid::new_v4(),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id.clone()
    }
}

trait EventContent: for<'a> Deserialize<'a> + Serialize + Clone {}
