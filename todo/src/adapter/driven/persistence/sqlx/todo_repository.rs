use async_trait::async_trait;
use rocket::futures::future::join_all;
use sqlx::query_builder::QueryBuilder;
use sqlx::{Postgres, Pool};
use uuid::Uuid;

use crate::application::port::driven::todo_repository::{TodoRepositoryTrait, UpdateTodo, FindTodo};
use crate::application::port::driven::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
    RepoUpdateError,
    RepoFindAllError,
};
use crate::domain::todo::Todo as TodoDomain;
use super::models::todo::Todo as TodoDB;


pub struct TodoRepository();

#[async_trait]
impl TodoRepositoryTrait<Pool<Postgres>> for TodoRepository {
    async fn find_by_id(&self, conn: &Pool<Postgres>, id: i32) -> Result<TodoDomain, RepoSelectError> {
        let todo = sqlx::query_as!(
            TodoDB,
            r#"
                SELECT * FROM todos WHERE id = $1
            "#,
            id
        ).fetch_one(conn).await;
        if let Err(err) = todo {
            return match err {
                sqlx::Error::RowNotFound => Err(RepoSelectError::NotFound),
                _ => Err(RepoSelectError::Unknown(err.to_string())),
            }
        }
        let todo = todo.unwrap();
        let tags = get_tags(conn, &todo.id).await.unwrap_or(Vec::new());
        Ok(todo.to_domain_todo(Some(tags)))
    }

    async fn find_all(
        &self, 
        conn: &Pool<Postgres>,
        user_id: &Uuid, 
        from: i64, 
        to: i64
    ) -> Result<Vec<TodoDomain>, RepoFindAllError> {
        let todos = sqlx::query_as!(
            TodoDB,
            r#"
                SELECT * FROM todos WHERE user_id = $1 ORDER BY id OFFSET $2 LIMIT $3
            "#,
            user_id,
            from,
            to
        ).fetch_all(conn).await;
        if let Ok(todos) = todos {
            let futures = todos.iter()
                .map(|todo| get_tags(conn, &todo.id));
            let every_tags = join_all(futures).await;
            Ok(todos.into_iter().zip(every_tags)
                .map(|(todo, tags)| {
                    todo.to_domain_todo(tags.ok())
                }).collect())
        } else if let Err(err) = todos {
            return match err {
                sqlx::Error::RowNotFound => Err(RepoFindAllError::NotFound),
                err => Err(RepoFindAllError::Unknown(Some(err.to_string()))),
            }
        } else {
            Err(RepoFindAllError::Unknown(None))
        }
    }

    async fn find_one_criteria(
        &self, 
        conn: &Pool<Postgres>,
        user_id: &Uuid, 
        find_todo: FindTodo
    ) -> Result<TodoDomain, RepoSelectError> {
        todo!();
    }

    async fn find_all_criteria(
        &self, conn: &Pool<Postgres>,
        user_id: &Uuid,
        from: i64, 
        to: i64, 
        find_todo: FindTodo
    ) -> Result<Vec<TodoDomain>, RepoFindAllError> {
        todo!();
    }

    async fn create(
        &self, 
        conn: &Pool<Postgres>,
        user_id: &Uuid, 
        todo: TodoDomain
    ) -> Result<TodoDomain, RepoCreateError> {
        todo!();
    }

    async fn update(
        &self, 
        conn: &Pool<Postgres>,
        todo: UpdateTodo
    ) -> Result<TodoDomain, RepoUpdateError> {
        todo!();
    }

    async fn add_tag(
        &self, 
        conn: &Pool<Postgres>,
        todo_id: i32, 
        tag: &String
    ) -> Result<TodoDomain, RepoUpdateError> {
        todo!();
    }

    async fn remove_tag(
        &self, 
        conn: &Pool<Postgres>,
        todo_id: i32, 
        tag: &String
    ) -> Result<TodoDomain, RepoUpdateError> {
        todo!();
    }

    async fn delete(&self, conn: &Pool<Postgres>,id: i32) -> Result<TodoDomain, RepoDeleteError> {
        todo!();
    }

    async fn delete_all_criteria(
        &self, conn: &Pool<Postgres>,
        find_todo: FindTodo
    ) -> Result<Vec<TodoDomain>, RepoDeleteError> {
        todo!();
    }
}

async fn get_tags(conn: &Pool<Postgres>, id: &i32) -> Result<Vec<String>, RepoSelectError> {
    let tags = sqlx::query!(
        r#"
            SELECT tg.tag_value
            FROM todos AS td
            JOIN todo_tag AS td_tg ON td_tg.todo_id = td.id
            JOIN tags AS tg ON tg.id = td_tg.tag_id
            WHERE td.id = $1;
        "#,
        id
    ).fetch_all(conn).await;
    match tags {
        Ok(tags) => Ok(tags.iter()
            .map(|tg| tg.tag_value.to_string()).collect()),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(RepoSelectError::NotFound),
            _ => Err(RepoSelectError::Unknown(err.to_string())),
        }
    }
}