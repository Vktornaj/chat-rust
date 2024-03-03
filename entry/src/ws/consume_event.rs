use axum::extract::ws::{Message, WebSocket};
use futures::SinkExt;
use futures_util::stream::SplitSink;
use protobuf::Message as ProtoMessage;
use tokio::time::sleep;
use uuid::Uuid;
use std::time::Duration;

use common::{
    adapter::state::PackageQueue, 
    domain::{
        models::client::Clients, protos_schemas::proto_package::{proto_package::Owner, ProtoPackage},
    }
};


pub async fn execute(
    clients: Clients<SplitSink<WebSocket, Message>>,
    event_queue: PackageQueue,
) {
    // Spawn a task to listen for updates to the event queue
    tokio::spawn(async move {
        loop {
            // Wait for the next event to be pushed to the queue
            let option_proto_package = {
                let mut queue = event_queue.write().await;
                queue.pop_front()
            };

            if let Some(proto_package) = option_proto_package {
                match send_package(proto_package.clone(), clients.clone()).await {
                    Ok(_) => println!("Message sent"),
                    Err(_) => println!("Error sending message"),
                }
            }
            // Sleep for a bit
            sleep(Duration::from_millis(100)).await;
        }
    });
}

pub async fn send_package(
    package: ProtoPackage, 
    clients: Clients<SplitSink<WebSocket, Message>>
) -> Result<(), String> {
    let recipient: Result<[u8; 16], _> = match package.owner.clone() {
        Some(Owner::Recipient(r)) => r.value.try_into(),
        _ => return Err("No recipient found".to_string()),
    };

    let recipient = if let Ok(recipient) = recipient {
        Uuid::from_bytes(recipient)
    } else {
        return Err("Invalid recipient".to_string());
    };

    let mut clients = clients.write().await;
    let client = match clients.get_mut(&recipient) {
        Some(c) => c,
        None => return Err("Client not found".to_string()),
    };

    if let Some(sender) = client.sender.as_mut() {
        sender.send(Message::Binary(package.write_to_bytes().unwrap())).await
            .map_err(|_| "".to_string())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {}
