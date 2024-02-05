use chrono::{Duration, TimeZone, Utc};
use common::domain::types::id::Id;
use jsonwebtoken as jwt;
use jwt::{DecodingKey, EncodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use jwt::{Algorithm, Validation};


#[derive(Debug)]
pub enum TokenDataError {
    Invalid(String),
    Expired(String),
    Unknown(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SingleUseTokenData {
    /// timestamp
    pub exp: i64,
    /// user id
    pub user_id: Uuid,
    /// token id
    pub tkn_id: Uuid,
}

impl SingleUseTokenData {
    pub fn new(user_id: &Id, duration: i64) -> Self {
        SingleUseTokenData {
            exp: (Utc::now() + Duration::minutes(duration)).timestamp(),
            user_id: user_id.to_owned().into(),
            tkn_id: Uuid::new_v4(),
        }
    }

    pub fn token(&self, secret: &[u8]) -> String {
        let encoding_key = EncodingKey::from_base64_secret(std::str::from_utf8(secret).unwrap());
        jwt::encode(&jwt::Header::default(), self, &encoding_key.unwrap()).expect("jwt")
    }

    pub fn from_token(token: &String, secret: &[u8]) -> Result<Self, TokenDataError> {
        if let Some(auth) = decode_token(token, secret) {
            if Utc::now() <= Utc.timestamp_opt(auth.exp, 0).unwrap() {
                Ok(auth)
            } else {
                println!("token error: Expired token");
                return Err(TokenDataError::Expired("Expired token".to_string()));
            }
        } else {
            println!("token error: Invalid token");
            return Err(TokenDataError::Invalid("Invalid token".to_string()));
        }
    }
}

/// Decode token into `Auth` struct. If any error is encountered, log it
/// an return None.
fn decode_token(token: &str, secret: &[u8]) -> Option<SingleUseTokenData> {
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