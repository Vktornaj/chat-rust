use axum::extract::ws::{WebSocket, Message};
use deadpool::managed::Pool;
use deadpool_redis::{Connection, Manager};
use futures_util::stream::SplitSink;
use sqlx::PgPool;

use crate::models::client::{Clients, EventQueue};

/// Debug only secret for JWT encoding & decoding.
pub const SECRET: &'static str = "8Xui8SN4mI+7egV/9dlfYYLGQJeEx4+DwmSQLwDVXJg=";

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S%.3fZ";

pub const TOKEN_PREFIX: &'static str = "Bearer ";

#[derive(Clone)]
pub enum Environment {
    Development,
    Production,
}

#[derive(Clone)]
pub struct Config {
    pub secret: Vec<u8>,
    pub environment: Environment,
}

#[derive(Clone)]
pub struct AppState {
    pub clients: Clients<SplitSink<WebSocket, Message>>,
    pub event_queue: EventQueue,
    pub db_sql_pool: PgPool,
    pub cache_pool: Pool<Manager, Connection>,
    pub config: Config,
}