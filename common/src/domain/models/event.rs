use std::{sync::Arc, collections::VecDeque};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::domain::types::recipient::Recipient;


// TODO: Build an adapter
pub type EventQueue<T> = Arc<RwLock<VecDeque<Event<T>>>>;

#[derive(Deserialize, Serialize, Clone)]
pub struct Event<T> {
    pub recipient_id: Recipient,
    pub content: EventContent<T>,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum EventContent<T> {
    Message(T),
    Notification,
}