use serde_json;
use axum::extract::ws::Message;

use crate::domain::models::message::Message as MyMessage;


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