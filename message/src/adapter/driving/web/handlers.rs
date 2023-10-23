use auth::domain::auth::Auth;
use axum::{
    extract::{ws::{WebSocketUpgrade, WebSocket, Message}, State},
    response::Response, TypedHeader, headers::{Authorization, authorization::Bearer},
};
use common::{
    config::AppState, 
    models::client::{Client, Event, EventContent},
    models::message_model::Message as MyMessage,
};
use futures_util::stream::StreamExt;
use uuid::Uuid;


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
        mut receiver
    ) = socket.split::<Message>();
    let client = Client {
        user_id,
        sender: Some(sender),
    };
    let client_id = Uuid::new_v4();
    state.clients.write().await.insert(client_id, client);
    tokio::spawn(async move {
        while let Some(message) = receiver.next().await {
            let message = if let Ok(message) = message{
                println!("Raw message: {:?}", &message);
                message
            } else {
                return;
            };

            let my_message = if let Ok(message) = MyMessage::try_from(message) {
                message
            } else {
                continue;
            };
    
            let event = Event {
                target_user_id: user_id,
                content: EventContent::Message(my_message),
            };
            state.event_queue.write().await.push_back(event);
        }
    });
}

