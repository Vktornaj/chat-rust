use std::{sync::Arc, collections::{HashMap, VecDeque}};
use aws_config::{meta::region::RegionProviderChain, Region};
use axum::extract::ws::{WebSocket, Message};
use deadpool_redis::Pool;
use futures_util::stream::SplitSink;
use sqlx::PgPool;
use tokio::sync::RwLock;
use aws_sdk_sesv2::Client;

use crate::domain::models::client::Clients;
use crate::domain::models::event::EventQueue;
use super::{config::Config, db, cache};


#[derive(Clone)]
pub struct AppState {
    pub clients: Clients<SplitSink<WebSocket, Message>>,
    pub event_queue: EventQueue,
    pub db_sql_pool: PgPool,
    pub cache_pool: Pool,
    pub email_conn: Client,
    pub config: Config,
}

impl AppState {
    pub async fn new() -> AppState {

        let region_provider = RegionProviderChain::first_try(Region::new("us-east-2"))
            .or_default_provider();
        let shared_config = aws_config::from_env().region(region_provider).load().await;

        AppState {
            db_sql_pool: db::create_pool().await,
            cache_pool: cache::create_pool().await,
            config: Config::new(),
            clients: Arc::new(RwLock::new(HashMap::new())),
            event_queue: Arc::new(RwLock::new(VecDeque::new())),
            email_conn: Client::new(&shared_config),
        }
    }
}
