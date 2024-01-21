use std::fmt::Display;

#[derive(Debug)]
pub enum RepoCreateError {
    Unknown(String),
}

impl Display for RepoCreateError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RepoCreateError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum RepoSelectError {
    NotFound,
    Unknown(String),
    ConnectionError(String),
}

#[derive(Debug)]
pub enum RepoUpdateError {
    NotFound,
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoDeleteError {
    NotFound,
    Unknown(String),
    SerializeError(String),
    ConnectionError(String),
}