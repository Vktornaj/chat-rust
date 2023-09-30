use serde::{Deserialize, Serialize};

use super::error::ErrorMsg;


#[derive(Clone, Deserialize, Serialize)]
pub struct Text(String);


impl TryFrom<String> for Text {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 0 && value.len() <= 140 {
            Ok(Self(value))
        } else {
            Err(ErrorMsg("Invalid text".to_string()))
        }
    }
}