use std::{sync::Arc, collections::{HashMap, VecDeque}};

use axum::extract::ws::{WebSocket, Message};
use deadpool_redis::Pool;
use futures_util::stream::SplitSink;
use sqlx::PgPool;
use tokio::sync::RwLock;

use crate::domain::models::client::{Clients, EventQueue};
use super::{config::Config, db, cache};
use crate::domain::models::message::Message as DomainMessage;


#[derive(Clone)]
pub struct AppState {
    pub clients: Clients<SplitSink<WebSocket, Message>>,
    pub event_queue: EventQueue<DomainMessage>,
    pub db_sql_pool: PgPool,
    pub cache_pool: Pool,
    pub config: Config,
}

impl AppState {
    pub async fn new() -> AppState {
        AppState {
            db_sql_pool: db::create_pool().await,
            cache_pool: cache::create_pool().await,
            config: Config::new().await,
            clients: Arc::new(RwLock::new(HashMap::new())),
            event_queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }
}
