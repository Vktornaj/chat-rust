use axum::extract::ws::{WebSocket, Message};
use common::domain::{models::{event::Event, message::Message as MessageDomain, client::Clients}, types::id::Id};
use futures_util::stream::SplitSink;

use crate::application::use_cases;


pub async fn send_message(event: Event<MessageDomain>, clients: Clients<SplitSink<WebSocket, Message>>) -> Result<Vec<Id>, ()> {
    match use_cases::send_message::execute(event, clients).await {
        Ok(ids) => Ok(ids),
        Err(_) => Err(()),
    }
}