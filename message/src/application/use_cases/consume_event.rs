// use common::domain::{
//     models::{client::Clients, event::Event},
//     types::{group::Group, recipient::Recipient, id::Id},
// };
// use futures_util::SinkExt;

// #[derive(Debug)]
// pub enum SendError {
//     InvalidData(String),
//     Unknown(String),
//     Conflict(String),
//     Unauthorized(String),
// }

// pub async fn execute<T, U>(event: Event, clients: Clients<T>) -> Result<Vec<Id>, SendError>
// where
//     T: 'static + futures_util::Sink<U> + Send + Unpin,
//     U: 'static + std::fmt::Debug + Send + Sync + Clone,
//     <T as futures_util::Sink<U>>::Error: Send,
// {
//     // Handle the event
//     let mut clients = clients.write().await;

//     let message: U = if let Ok(message) = U::try_from(event.content.clone()) {
//         message
//     } else {
//         return Err(SendError::InvalidData("".to_string()));
//     };

//     let ids = match event.recipient_id {
//         Recipient::User(user_id) => vec![user_id],
//         Recipient::Group(Group { members, .. }) => members.into_iter().collect(),
//     };

//     let futures_ = clients
//         .iter_mut()
//         .map(|(_, client)| async {
//             if !ids.contains(&client.user_id) {
//                 return None;
//             }
//             if let Some(f) = client.sender.as_mut().map(|x| x.send(message.clone())) {
//                 match f.await {
//                     Ok(_) => None,
//                     Err(_) => Some(client.user_id)
//                 }
//             } else {
//                 Some(client.user_id)
//             }
//         });

//     Ok(futures::future::join_all(futures_)
//         .await
//         .into_iter()
//         .flatten()
//         .collect::<Vec<Id>>())
// }

// #[cfg(test)]
// mod tests {}
