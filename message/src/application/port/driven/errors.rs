use std::fmt;

#[derive(Debug)]
pub enum QueueAddError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}

#[derive(Debug)]
pub enum MediaError {
    InvalidData(String),
    Unknown(String),
    Conflict(String),
}

impl fmt::Display for MediaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl fmt::Display for QueueAddError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}