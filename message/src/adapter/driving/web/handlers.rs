use auth::domain::auth::Auth;
use axum::{
    extract::{ws::{WebSocketUpgrade, WebSocket, Message}, State},
    response::Response, TypedHeader, headers::{Authorization, authorization::Bearer},
};
use common::{
    config::AppState, 
    models::{client::{Client, EventQueue, Event}, message_model::MessageContent}, 
    types::{sender_type::Sender, id::Id, recipient::Recipient, text::Text}
};
use futures_util::{stream::{StreamExt, SplitSink, SplitStream}, SinkExt};
use uuid::Uuid;
use common::models::message_model::Message as MessageModel;


pub async fn ws_handler(
    ws: WebSocketUpgrade, 
    State(state): State<AppState<Message>>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(
        socket, 
        state, 
        token.token().to_string())
    )
}

async fn handle_socket(socket: WebSocket, state: AppState<Message>, token: String) {
    let user_id = Auth::from_token(&token, &state.config.secret).unwrap().id;
    let (
        sender, 
        receiver
    ) = socket.split::<Message>();

    tokio::spawn(read(receiver, user_id, state.event_queue));

    let client = Client {
        user_id,
        topics: vec!["test".to_string()],
        sender: Some(sender),
    };
    let client_id = Uuid::new_v4();
    state.clients.write().await.insert(client_id, client);
}

async fn read(mut receiver: SplitStream<WebSocket>, user_id: Uuid, event_queue: EventQueue) {
    while let Some(msg) = receiver.next().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };
        println!("Received a message: {:?} from: {}", &msg, &user_id);
        event_queue.write().await.push(Event {
            user_id: Some(user_id),
            message: MessageModel::new(
                Sender::User(Id::try_from(user_id).unwrap()), 
                Recipient::User(Id::try_from(Uuid::new_v4()).unwrap()), 
                MessageContent::Text(Text::try_from(msg.into_text().unwrap()).unwrap()),
            ),
        });
    }

    // write(sender)
}

async fn write(mut sender: SplitSink<WebSocket, Message>) {
    let n_msg = 20;
    for i in 0..n_msg {
        // In case of any websocket error, we exit.
        if sender
            .send(Message::Text(format!("Message #{}", i)))
            .await
            .is_err()
        {
            return;
        }

        tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    }
}