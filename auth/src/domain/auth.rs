use jsonwebtoken as jwt;
use jwt::{EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Duration, Utc, TimeZone};


#[derive(Debug)]
pub enum AuthError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Auth {
    /// timestamp
    pub exp: i64,
    /// user id
    pub username: String,
}

impl Auth {
    pub fn new(username: &String) -> Self {
        Auth { 
            exp: (Utc::now() + Duration::days(60)).timestamp(), 
            username: username.to_owned() 
        }
    }

    pub fn token(&self, secret: &[u8]) -> String {
        let encoding_key = EncodingKey::from_base64_secret(
            std::str::from_utf8(secret).unwrap()
        );
        jwt::encode(&jwt::Header::default(), self, &encoding_key.unwrap())
            .expect("jwt")
    }

    // TODO: Determinate if token is valid by date
    pub fn from_token(token: &String, secret: &[u8]) -> Result<Self, AuthError> {
        if let Some(auth) = decode_token(token, secret) {
            if Utc::now() <= Utc.timestamp_opt(auth.exp, 0).unwrap() {
                Ok(auth)
            } else {
                println!("token error: Expired token");
                return Err(AuthError::InvalidData("Expired token".to_string()));
            }
        } else {
            println!("token error: Invalid token");
            return Err(AuthError::InvalidData("Invalid token".to_string()));
        }
    }
}

/// Decode token into `Auth` struct. If any error is encountered, log it
/// an return None.
fn decode_token(token: &str, secret: &[u8]) -> Option<Auth> {
    use jwt::{Algorithm, Validation};

    let decoding_key = DecodingKey::from_base64_secret(
        std::str::from_utf8(secret).unwrap()
    );

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