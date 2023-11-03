use futures_util::{Stream, StreamExt, stream::SplitSink};
use uuid::Uuid;

use common::domain::{models::{
        client::{Client, Clients},
        event::{Event, EventContent, EventQueue}, 
        message::Message as MessageDomain,
    }, 
    types::id::Id
};


pub async fn execute<T, U, E>(
    clients: Clients<SplitSink<T, U>>,
    event_queue: EventQueue<MessageDomain>,
    recipient_id: Uuid,
    socket: T,
) where 
    T: 'static + Stream<Item = Result<U, E>> + futures_util::Sink<U> + Send,
    U: std::fmt::Debug + Send,
    E: std::fmt::Debug + Send,
    MessageDomain: std::convert::TryFrom<U, Error = String>,
{
    let (sender, mut receiver) = socket.split::<U>();
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
                content: EventContent::Message(message_domain) 
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