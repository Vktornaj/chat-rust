use std::{sync::Arc, collections::{HashMap, VecDeque}};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::types::{recipient::Recipient, id::Id};


#[derive(Debug)]
pub struct Client<T> {
    pub user_id: Id,
    pub sender: Option<T>,
    pub task: tokio::task::JoinHandle<()>,
}

pub type Clients<T> = Arc<RwLock<HashMap<Uuid, Client<T>>>>;
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