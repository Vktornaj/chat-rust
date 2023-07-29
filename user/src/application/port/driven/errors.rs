#[derive(Debug)]
pub enum RepoCreateError {
    InvalidData(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoSelectError {
    NotFound,
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoFindAllError {
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoUpdateError {
    InvalidData(String),
    NotFound,
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoDeleteError {
    NotFound,
    InvalidData(String),
    Unknown(String),
}

#[derive(Debug)]
pub enum RepoInitializeError {
    NotFound,
    Unknown(String),
}