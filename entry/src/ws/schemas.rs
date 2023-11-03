use serde::Deserialize;
use uuid::Uuid;


pub struct ExtractError {
    pub message: String,
}

#[derive(Deserialize)]
pub enum EventTypeJson {
    Message,
    Notification,
}

#[derive(Deserialize)]
pub enum EventContentJson {
    Message(MessageJson),
    Notification(NotificationJson),
}

#[derive(Deserialize)]
pub struct MessageJson {
    pub recipient: Uuid,
    pub content: String,
}

#[derive(Deserialize)]
pub struct NotificationJson {
    pub recipient: Uuid,
    pub content: String,
}

#[derive(Deserialize)]
pub struct EventJson {
    pub event_type: EventTypeJson,
    pub event_content: String, // MessageJson or NotificationJson
}

