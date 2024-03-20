use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthWebSocket {
    pub auth_token: String,
}