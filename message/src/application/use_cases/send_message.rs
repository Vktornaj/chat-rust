use common::domain::{
    models::{client::Clients, event::Event, message::Message as MessageDomain},
    types::{group::Group, recipient::Recipient, id::Id},
};
use futures_util::{SinkExt, Stream};

#[derive(Debug)]
pub enum SendError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
    Unautorized(String),
}

pub async fn execute<T, U, E>(event: Event<MessageDomain>, clients: &Clients<T>) -> Result<Vec<Id>, SendError>
where
    T: 'static + Stream<Item = Result<U, E>> + futures_util::Sink<U> + Send + Unpin,
    U: 'static + TryFrom<MessageDomain, Error = String> + std::fmt::Debug + Send + Sync + Clone,
    E: std::fmt::Debug + Send,
    MessageDomain: TryFrom<U, Error = String>,
    <T as futures_util::Sink<U>>::Error: Send,
{
    // Handle the event
    let mut clients = clients.write().await;

    let message: U = if let Ok(message) = U::try_from(event.content.clone()) {
        message
    } else {
        return Err(SendError::InvalidData("".to_string()));
    };

    let ids = match event.recipient_id {
        Recipient::User(user_id) => vec![user_id],
        Recipient::Group(Group { members, .. }) => members.into_iter().collect(),
    };

    let futures_ = clients
        .iter_mut()
        .map(|(_, client)| async {
            if !ids.contains(&client.user_id) {
                return None;
            }
            if let Some(f) = client.sender.as_mut().map(|x| x.send(message.clone())) {
                f.await.map(|_| client.user_id).ok()
            } else {
                None
            }
        });

    Ok(futures::future::join_all(futures_)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<Id>>())
}

#[cfg(test)]
mod tests {}
