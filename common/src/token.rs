use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};

use crate::config;


#[derive(Debug, Deserialize, Serialize)]
pub struct Token {
    pub value: String
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Token {
    type Error = ();

    /// Extract token from the "Authorization" header.
    ///
    /// Handlers with Token guard will fail with 503 error.
    /// Handlers with Option<Token> will be called with None.
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Token, Self::Error> {
        // let state = req.rocket().state::<AppState>().unwrap();
        if let Some(token) = extract_token_from_request(req) {
            Outcome::Success(Token { value: token })
        } else {
            Outcome::Failure((Status::Forbidden, ()))
        }
    }
}

fn extract_token_from_request(request: &Request) -> Option<String> {
    let r = request
        .headers()
        .get_one("authorization")
        .and_then(extract_token_from_header);
    match r {
        Some(_) => Some(r.unwrap().to_owned()),
        None => None,
    }    
}

fn extract_token_from_header(header: &str) -> Option<&str> {
    if header.starts_with(config::TOKEN_PREFIX) {
        Some(&header[config::TOKEN_PREFIX.len()..])
    } else {
        None
    }
}
