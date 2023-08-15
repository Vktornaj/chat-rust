#[derive(Debug)]
pub enum RepoCreateError {
    Unknown(String),
    Conflict(String),
}

#[derive(Debug)]
pub enum RepoSelectError {
    NotFound,
    Unknown(String),
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
}

// #[derive(Debug)]
// pub enum RepoInitializeError {
//     NotFound,
//     Unknown(String),
// }