use deadpool::managed::Pool;
use deadpool_redis::{Connection, Manager};
use sqlx::PgPool;
use std::env;

use crate::{cache, db};

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
    pub db_sql_pool: PgPool,
    pub cache_pool: Pool<Manager, Connection>,
    pub config: Config,
}

impl AppState {
    pub async fn new() -> AppState {
        let secret = env::var("SECRET_KEY").unwrap_or_else(|err| {
            if cfg!(debug_assertions) {
                SECRET.to_string()
            } else {
                panic!("No SECRET_KEY environment variable found: {:?}", err)
            }
        });

        let environment = match env::var("ROCKET_ENV")
            .unwrap_or_else(|_| "development".to_string())
            .as_str()
        {
            "development" => Environment::Development,
            "production" => Environment::Production,
            s => panic!("Unknown environment: {}", s),
        };

        AppState {
            db_sql_pool: db::create_pool().await,
            cache_pool: cache::create_pool().await,
            config: Config {
                secret: secret.into_bytes(),
                environment,
            },
        }
    }
}
