use futures_util::{Stream, stream::SplitSink};
use uuid::Uuid;

use common::domain::models::{
    client::Clients, 
    message::Message as MyMessage
};


pub async fn execute<T, U, E>(
    clients: Clients<SplitSink<T, U>>,
    client_id: Uuid,
) where 
    T: 'static + Stream<Item = Result<U, E>> + futures_util::Sink<U> + Send,
    U: std::fmt::Debug + Send,
    E: std::fmt::Debug + Send,
    MyMessage: std::convert::TryFrom<U, Error = String>,
{
    clients.write().await.remove(&client_id);
}

#[cfg(test)]
mod tests {}