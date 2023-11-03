use std::{sync::Arc, collections::HashMap};
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::types::id::Id;


pub type Clients<T> = Arc<RwLock<HashMap<Uuid, Client<T>>>>;

#[derive(Debug)]
pub struct Client<T> {
    pub user_id: Id,
    pub sender: Option<T>,
    pub task: tokio::task::JoinHandle<()>,
}