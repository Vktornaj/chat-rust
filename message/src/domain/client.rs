use std::{sync::{Arc, RwLock}, collections::HashMap};
use serde::{Deserialize, Serialize};

use super::message::Message;

#[derive(Debug, Clone)]
pub struct Client<T> {
    pub user_id: usize,
    pub topics: Vec<String>,
    pub sender: Option<T>,
}

pub type Clients<T> = Arc<RwLock<HashMap<String, Client<T>>>>;

#[derive(Deserialize, Serialize)]
pub struct Event {
    user_id: Option<usize>,
    message: Message,
}