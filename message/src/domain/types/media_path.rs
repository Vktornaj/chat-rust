use serde::{Deserialize, Serialize};

use super::error::ErrorMsg;


#[derive(Clone, Deserialize, Serialize)]
pub struct MediaPath(String);


impl TryFrom<String> for MediaPath {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() > 0 {
            Ok(Self(value))
        } else {
            Err(ErrorMsg("Invalid text".to_string()))
        }
    }
}