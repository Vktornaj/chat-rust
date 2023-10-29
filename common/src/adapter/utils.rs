use std::{env, sync::Arc, collections::{HashMap, VecDeque}};
use tokio::sync::RwLock;
use serde_json;
use axum::extract::ws::Message;

use crate::models::message::Message as MyMessage;
use crate::{
    config::{AppState, SECRET, Environment, Config}, 
    db, 
    cache
};


impl AppState {
    pub async fn new() -> AppState {
        let secret = env::var("SECRET_KEY").unwrap_or_else(|err| {
            if cfg!(debug_assertions) {
                SECRET.to_string()
            } else {
                panic!("No SECRET_KEY environment variable found: {:?}", err)
            }
        });

        let environment = match env::var("ENVIRONMENT")
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
            clients: Arc::new(RwLock::new(HashMap::new())),
            event_queue: Arc::new(RwLock::new(VecDeque::new())),
        }
    }
}

impl TryFrom<MyMessage> for Message {
    type Error = String;

    fn try_from(value: MyMessage) -> Result<Self, Self::Error> {
        // serialize value
        serde_json::to_string(&value)
            .map_err(|err| format!("Error serializing message: {:?}", err))
            .map(|value| Message::Text(value))
    }
}

impl TryFrom<Message> for MyMessage {
    type Error = String;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        match value {
            Message::Text(value) => serde_json::from_str(&value)
                .map_err(|err| format!("Error deserializing message: {:?}", err)),
            _ => Err("Error deserializing message: not a text message".to_string()),
        }
    }
}