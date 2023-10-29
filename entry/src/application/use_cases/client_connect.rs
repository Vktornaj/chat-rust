use futures_util::{Stream, StreamExt, stream::SplitSink};
use uuid::Uuid;

use common::models::{
    client::{Client, EventContent, Event, Clients, EventQueue}, 
    message::Message as MyMessage
};


pub async fn execute<T, U, E>(
    clients: Clients<SplitSink<T, U>>,
    event_queue: EventQueue,
    user_id: Uuid,
    socket: T,
) where 
    T: 'static + Stream<Item = Result<U, E>> + futures_util::Sink<U> + Send,
    U: std::fmt::Debug + Send,
    E: std::fmt::Debug + Send,
    MyMessage: std::convert::TryFrom<U, Error = String>,
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

            let my_message = if let Ok(message) = MyMessage::try_from(message) {
                message
            } else {
                eprintln!("Message extraction error");
                continue;
            };
    
            let event = Event {
                target_user_id: user_id,
                content: EventContent::Message(my_message),
            };
            event_queue.write().await.push_back(event);
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