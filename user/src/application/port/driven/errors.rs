#[derive(Debug)]
pub enum RepoCreateError {
    Unknown(String),
    Conflict(String),
    SerializeError(String),
    ConnectionError(String),
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