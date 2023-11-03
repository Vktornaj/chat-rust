use axum::extract::ws::{WebSocket, Message};
use futures_util::{StreamExt, stream::SplitSink};
use uuid::Uuid;

use common::domain::{models::{
        client::{Client, Clients},
        event::{Event, EventQueue}, 
        message::Message as MessageDomain,
    }, 
    types::id::Id
};


pub async fn execute(
    clients: Clients<SplitSink<WebSocket, Message>>,
    event_queue: EventQueue<MessageDomain>,
    recipient_id: Uuid,
    socket: WebSocket,
) {
    let (sender, mut receiver) = socket.split();
    // Create a new client id
    let client_id = Uuid::new_v4();
    // 
    let task = async move {
        while let Some(message) = receiver.next().await {
            let message = if let Ok(message) = message {
                message
            } else {
                eprintln!("Message error");
                return;
            };

            let message_domain: MessageDomain = if let Ok(message) = MessageDomain::try_from(message) {
                message
            } else {
                eprintln!("Message extraction error");
                continue;
            };

            event_queue.write().await.push_back(Event {
                recipient_id: message_domain.recipient.clone(), 
                content: message_domain 
            });
        }
    };
    let user_id = if let Ok(id) = Id::try_from(recipient_id) {
        id
    } else {
        return;
    };
    let client = Client {
        user_id,
        sender: Some(sender),
        task: tokio::spawn(task),
    };
    // Add the client to the hashmap
    clients.write().await.insert(client_id, client);
}

#[cfg(test)]
mod tests {}