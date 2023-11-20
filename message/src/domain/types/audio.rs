use common::domain::types::error::ErrorMsg;


pub struct Audio(Vec<u8>);

impl TryFrom<Vec<u8>> for Audio {
    type Error = ErrorMsg;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > 0 {
            Ok(Self(value))
        } else {
            Err(ErrorMsg("Invalid text".to_string()))
        }
    }
}