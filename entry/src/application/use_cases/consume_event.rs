use futures_util::{Stream, stream::SplitSink, SinkExt};

use common::domain::{models::{
    client::{EventContent, Clients, EventQueue}, 
    message::Message as MessageDomain,
}, types::id::Id};


pub async fn execute<T, U, E>(
    clients: Clients<SplitSink<T, U>>,
    event_queue: EventQueue<MessageDomain>,
) where 
    T: 'static + Stream<Item = Result<U, E>> + futures_util::Sink<U> + Send,
    U: 'static + TryFrom<MessageDomain, Error = String> + std::fmt::Debug + Send + Sync,
    E: std::fmt::Debug + Send,
    MessageDomain: TryFrom<U, Error = String>,
    <T as futures_util::Sink<U>>::Error: Send,
{
    // Spawn a task to listen for updates to the event queue
    tokio::spawn(async move {
        loop {
            // Wait for the next event to be pushed to the queue
            let event = {
                let mut queue = event_queue.write().await;
                queue.pop_front()
            };

            if let Some(event) = event {
                // Handle the event
                let mut clients = clients.write().await;
                let f = clients
                    .iter_mut()
                    .map(|(_, client)| {
                        if Into::<Id>::into(event.recipient_id.clone()) != client.user_id {
                            return None;
                        }
                        let message: U = match &event.content {
                            EventContent::Message(message) => {
                                if let Ok(message) = U::try_from(message.clone()) {
                                    message
                                } else {
                                    return None;
                                }
                            },
                            EventContent::Notification => {
                                return None;                
                            },
                        };
                        client.sender.as_mut().map(|x| x.send(message))
                    })
                    .filter_map(|x| x);
                futures::future::join_all(f).await;
            }
        }
    });
}

#[cfg(test)]
mod tests {}