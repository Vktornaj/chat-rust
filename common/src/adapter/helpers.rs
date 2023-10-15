use std::{env, sync::Arc, collections::HashMap};

use axum::extract::ws::Message;
use tokio::sync::RwLock;

use crate::{
    models::message_model::MessageContent, 
    types::error::ErrorMsg, config::{AppState, SECRET, Environment, Config}, db, cache
};


impl TryFrom<MessageContent> for Message {
    type Error = ErrorMsg;

    fn try_from(value: MessageContent) -> Result<Self, Self::Error> {
        match value {
            MessageContent::Text(text) => Ok(Message::Text(text.into())),
            MessageContent::Image(image) => Ok(Message::Binary(image)),
            MessageContent::Video(video) => Ok(Message::Binary(video)),
            MessageContent::Audio(audio) => Ok(Message::Binary(audio)),
            MessageContent::File(file) => Ok(Message::Binary(file)),
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
            clients: Arc::new(RwLock::new(HashMap::new())),
            event_queue: Arc::new(RwLock::new(Vec::new())),
        }
    }
}