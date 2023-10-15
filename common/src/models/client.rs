use std::{sync::Arc, collections::HashMap};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use uuid::Uuid;

use super::message_model::Message;


#[derive(Debug, Clone)]
pub struct Client<T> {
    pub user_id: Uuid,
    pub topics: Vec<String>,
    pub sender: Option<T>,
}

pub type Clients<T> = Arc<RwLock<HashMap<Uuid, Client<T>>>>;
pub type EventQueue = Arc<RwLock<Vec<Event>>>;

#[derive(Deserialize, Serialize, Clone)]
pub struct Event {
    pub user_id: Option<Uuid>,
    pub message: Message,
}