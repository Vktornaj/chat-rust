use axum::extract::ws::{Message, WebSocket};
use futures_util::stream::SplitSink;

use common::domain::{
    models::{client::Clients, event::EventQueue, message::Message as MessageDomain},
    types::recipient::Recipient,
};
use message::handlers;

pub async fn execute(
    clients: Clients<SplitSink<WebSocket, Message>>,
    event_queue: EventQueue<MessageDomain>,
) {
    // Spawn a task to listen for updates to the event queue
    tokio::spawn(async move {
        loop {
            // Wait for the next event to be pushed to the queue
            let event = {
                let mut queue = event_queue.write().await;
                queue.pop_front()
            };

            if let Some(mut event) = event {
                match handlers::send_message(event.clone(), clients.clone()).await {
                    Ok(unsent_ids) => {
                        if !unsent_ids.is_empty() {
                            event.recipient_id = match event.recipient_id {
                                Recipient::User(_) => Recipient::User(unsent_ids[0]),
                                Recipient::Group(group) => {
                                    let mut group_clone = group.clone();
                                    group_clone.members = unsent_ids;
                                    return Recipient::Group(group_clone);
                                }
                            };

                            event_queue.write().await.push_back(event);
                        }
                    }
                    Err(_) => println!("Error sending message: {}", &event.content.id),
                }
            }
        }
    });
}

#[cfg(test)]
mod tests {}