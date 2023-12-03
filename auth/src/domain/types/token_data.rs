use chrono::{Duration, TimeZone, Utc};
use jsonwebtoken as jwt;
use jwt::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use jwt::{Algorithm, Validation};


#[derive(Debug)]
pub enum TokenDataError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TokenData {
    /// timestamp
    pub exp: i64,
    /// user id
    pub id: Uuid,
    /// token id
    pub tkn_id: Option<Uuid>,
}

impl TokenData {
    pub fn new(id: &Uuid) -> Self {
        TokenData {
            exp: (Utc::now() + Duration::days(60)).timestamp(),
            id: id.to_owned(),
            tkn_id: Some(Uuid::new_v4()),
        }
    }

    pub fn new_reset_password_token(id: &Uuid) -> Self {
        TokenData {
            exp: (Utc::now() + Duration::minutes(15)).timestamp(),
            id: id.to_owned(),
            tkn_id: None,
        }
    }

    pub fn token(&self, secret: &[u8]) -> String {
        let encoding_key = EncodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());
        jwt::encode(&jwt::Header::default(), self, &encoding_key.unwrap()).expect("jwt")
    }

    // TODO: Determinate if token is valid by date
    pub fn from_token(token: &String, secret: &[u8]) -> Result<Self, TokenDataError> {
        if let Some(auth) = decode_token(token, secret) {
            if Utc::now() <= Utc.timestamp_opt(auth.exp, 0).unwrap() {
                Ok(auth)
            } else {
                println!("token error: Expired token");
                return Err(TokenDataError::InvalidData("Expired token".to_string()));
            }
        } else {
            println!("token error: Invalid token");
            return Err(TokenDataError::InvalidData("Invalid token".to_string()));
        }
    }
}

/// Decode token into `Auth` struct. If any error is encountered, log it
/// an return None.
fn decode_token(token: &str, secret: &[u8]) -> Option<TokenData> {
    let decoding_key = DecodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());

    jwt::decode(
        token,
        &decoding_key.unwrap(),
        &Validation::new(Algorithm::HS256),
    )
        .map_err(|err| {
            eprintln!("Auth decode error: {:?}", err);
        })
        .ok()
        .map(|token_data| token_data.claims)
}