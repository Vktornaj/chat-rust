use chrono::{DateTime, Utc};
use async_trait::async_trait;


use super::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoFindAllError, 
    RepoSelectError, 
    RepoUpdateError
};
use crate::domain::todo::{Todo, Status};


pub struct FindTodo {
    pub username: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
    pub tags: Option<Vec<String>>,
}

// pub struct FindTodoByDateRange {
//     pub create_date: Option<(DateTime<Utc>, DateTime<Utc>)>,
//     pub done_date: Option<(DateTime<Utc>, DateTime<Utc>)>,
//     pub deadline: Option<(DateTime<Utc>, DateTime<Utc>)>,
// }

pub struct UpdateTodo {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
    pub done_date: Option<Option<DateTime<Utc>>>,
    pub deadline: Option<Option<DateTime<Utc>>>,
}

#[async_trait]
pub trait TodoRepository<T> {
    /// Find and return one single record from the persistence system
    async fn find_one(&self, conn: &T, id: i32) -> Result<Todo, RepoSelectError>;

    /// Find and return all records corresponding to the user
    async fn find_all(
        &self, 
        conn: &T, 
        username: &String, 
        from: i32, 
        to: i32
    ) -> Result<Vec<Todo>, RepoFindAllError>;
    
    /// Find and return one single record from the persistence system
    async fn find_one_criteria(
        &self, 
        conn: &T, 
        username: &String, 
        find_todo: FindTodo
    ) -> Result<Todo, RepoSelectError>;

    /// Find and return all records corresponding to the search criteria from the persistence system
    async fn find_all_criteria(
        &self, conn: &T, 
        username: &String,
        from: i32, 
        to: i32, 
        find_todo: FindTodo
    ) -> Result<Vec<Todo>, RepoFindAllError>;
    
    /// Find and return all records corresponding to the search criteria from the persistence system
    // async fn find_all_date_range(
    //     &self, conn: &T, 
    //     username: &String, 
    //     from: i32, 
    //     to: i32,
    //     find_todo_by_date_range: FindTodoByDateRange
    // ) -> Result<Vec<Todo>, RepoFindAllError>;

    /// Insert the received entity in the persistence system
    async fn create(&self, conn: &T, username: &String, todo: Todo) -> Result<Todo, RepoCreateError>;

    /// Update one single record already present in the persistence system
    async fn update(
        &self, 
        conn: &T, 
        todo: UpdateTodo
    ) -> Result<Todo, RepoUpdateError>;
    
    /// Update one single record already present in the persistence system
    async fn add_tag(
        &self, 
        conn: &T, 
        todo_id: i32, 
        tag: &String
    ) -> Result<Todo, RepoUpdateError>;
    
    /// Update one single record already present in the persistence system
    async fn remove_tag(
        &self, 
        conn: &T, 
        todo_id: i32, 
        tag: &String
    ) -> Result<Todo, RepoUpdateError>;

    /// Delete one single record from the persistence system
    async fn delete(&self, conn: &T, id: i32) -> Result<Todo, RepoDeleteError>;
    
    /// Delete one single record from the persistence system
    async fn delete_all_criteria(
        &self, conn: &T, 
        find_todo: FindTodo
    ) -> Result<Vec<Todo>, RepoDeleteError>;
}