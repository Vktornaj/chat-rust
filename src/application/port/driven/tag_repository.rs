use crate::domain::tag::Tag;
use super::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoFindAllError, 
    RepoSelectError, 
    RepoUpdateError
};


pub trait TagRepository {
    /// Find and return one single record from the persistence system
    fn find_one(&self, id: i32) -> Result<Tag, RepoSelectError>;

    /// Find and return all records corresponding to the search criteria from the persistence system
    fn find_all(&self, usernmae: &String, from: i64, to: i64) -> Result<Vec<Tag>, RepoFindAllError>;
    
    /// Insert the received entity in the persistence system
    fn create(&self, Tag: Tag) -> Result<Tag, RepoCreateError>;

    /// Update one single record already present in the persistence system
    fn update(&self, Tag: Tag) -> Result<Tag, RepoUpdateError>;

    /// Delete one single record from the persistence system
    fn delete(&self, id: i32) -> Result<(), RepoDeleteError>;
}