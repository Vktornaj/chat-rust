use std::{env, sync::Arc, collections::{HashMap, VecDeque}};

use axum::extract::ws::Message;
use tokio::{sync::RwLock, runtime::Builder, task::spawn_blocking};

use crate::{
    models::message_model::Message as MyMessage, 
    types::error::ErrorMsg, config::{AppState, SECRET, Environment, Config}, db, cache
};


impl TryFrom<MyMessage> for Message {
    type Error = ErrorMsg;

    fn try_from(value: MyMessage) -> Result<Self, Self::Error> {
        // serialize value
        serde_json::to_string(&value)
            .map_err(|err| ErrorMsg(format!("Error serializing message: {:?}", err)))
            .map(|value| Message::Text(value))
    }
}

impl TryFrom<Message> for MyMessage {
    type Error = ErrorMsg;

    fn try_from(value: Message) -> Result<Self, Self::Error> {
        match value {
            Message::Text(value) => serde_json::from_str(&value)
                .map_err(|err| ErrorMsg(format!("Error deserializing message: {:?}", err))),
            _ => Err(ErrorMsg("Error deserializing message: not a text message".to_string())),
        }
    }
}

impl AppState<Message> {
    pub async fn new() -> AppState<Message> {
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