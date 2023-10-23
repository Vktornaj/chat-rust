use std::{sync::Arc, collections::{HashMap, VecDeque}};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::message_model::Message;


#[derive(Debug, Clone)]
pub struct Client<T> {
    pub user_id: Uuid,
    pub sender: Option<T>,
}

pub type Clients<T> = Arc<RwLock<HashMap<Uuid, Client<T>>>>;
pub type EventQueue = Arc<RwLock<VecDeque<Event>>>;

#[derive(Deserialize, Serialize, Clone)]
pub struct Event {
    pub target_user_id: Uuid,
    pub content: EventContent,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum EventContent {
    Message(Message),
    Notification,
}