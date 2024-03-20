use axum::extract::ws::{WebSocket, Message};
use protobuf;
use futures_util::{StreamExt, stream::SplitSink};
use uuid::Uuid;

use common::{
    adapter::state::PackageQueue, 
    domain::{
        models::client::{Client, Clients}, 
        types::id::Id
    }
};
use common::domain::protos_schemas::proto_package::ProtoPackage;


pub async fn execute(
    clients: Clients<SplitSink<WebSocket, Message>>,
    package_queue: PackageQueue,
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
    
    // Create events
    let task = async move {
        while let Some(message) = receiver.next().await {
            println!("Message: {:?}", message);
            let message = if let Ok(message) = message {
                message
            } else {
                eprintln!("Message error");
                return;
            };

            let proto_package: ProtoPackage = match message {
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

            if proto_package.package_type == String::from("MESSAGE") {
                package_queue.write().await.push_back(proto_package);
            } else {
                eprintln!("Message error");
                continue;
            }
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