use axum::extract::ws::{WebSocket, Message};
use protobuf;
use futures_util::{StreamExt, stream::SplitSink};
use uuid::Uuid;

use common::domain::{models::{
        client::{Client, Clients},
        event::{Event, EventQueue}, 
        message::Message as MessageDomain,
    }, 
    types::id::Id
};
use common::adapter::protos_schemas::proto_message::ProtoMessage;


pub async fn execute(
    clients: Clients<SplitSink<WebSocket, Message>>,
    event_queue: EventQueue<MessageDomain>,
    sender_id: Uuid,
    socket: WebSocket,
) {
    let (sender, mut receiver) = socket.split();
    // Create a new client id
    let client_id = Uuid::new_v4();

    let user_id: Id = if let Ok(user_id) = sender_id.try_into() {
        user_id
    } else {
        eprintln!("Error converting Uuid to Id");
        return;
    };
    
    let task = async move {
        while let Some(message) = receiver.next().await {
            let message = if let Ok(message) = message {
                message
            } else {
                eprintln!("Message error");
                return;
            };

            let proto_message: ProtoMessage = match message {
                Message::Binary(bytes) => {
                    if let Ok (proto_message) = protobuf::Message::parse_from_bytes(&bytes) {
                        proto_message
                    } else {
                        eprintln!("Message error");
                        continue;
                    }
                },
                _ => {
                    eprintln!("Message error");
                    continue;
                }
            };

            let message_domain = MessageDomain::new(
                user_id.clone().into(),
                if let Some(recipient) = proto_message.recipient {
                    if let Ok(recipient) = recipient.try_into() {
                        recipient
                    } else {
                        eprintln!("Error converting recipient");
                        continue;
                    }
                } else {
                    eprintln!("Error getting recipient");
                    continue;
                },
                proto_message.content,
            );

            event_queue.write().await.push_back(Event {
                recipient_id: message_domain.recipient.clone(), 
                content: message_domain 
            });
        }
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