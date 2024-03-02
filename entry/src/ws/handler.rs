use auth::{authenticate_single_use_token, TokenCache};
use axum::{extract::{Query, State, WebSocketUpgrade}, http::StatusCode, response::{IntoResponse, Response}};
use common::adapter::state::AppState;

use crate::schemas::AuthWebSocket;

use super::client_connect;


// Websocket handlers
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(auth_websocket): Query<AuthWebSocket>,
) -> Response {
    let user_id = if let Ok(token_data) = authenticate_single_use_token::execute(
        &state.config.secret,
        &state.cache_pool,
        &TokenCache(),
        auth_websocket.auth_token,
    ).await {
        token_data.user_id
    } else {
        return StatusCode::UNAUTHORIZED.into_response();
    };
    ws.on_upgrade(move |socket| {
        client_connect::execute(state.clients, state.package_queue, user_id, socket)
    })
}


// Event queue
// async fn run_consumer_event_queue(
//     event_queue: EventQueue<MessageDomain>,
//     clients: Clients<SplitSink<WebSocket, Message>>,
// ) {
//     ws::consume_event::execute(clients, event_queue).await;
// }
