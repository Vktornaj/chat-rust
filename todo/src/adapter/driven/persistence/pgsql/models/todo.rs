use chrono::{DateTime, Utc};
use diesel::{prelude::*};

use super::super::schema::{_todo, _todo_tag};
use crate::domain::todo::{Todo as TodoDomain, Status};


#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = _todo)]
pub struct Todo {
    pub id: i32,
    pub username: String,
    pub title: String,
    pub description: Option<String>,
    pub status: i32,
    pub create_date: DateTime<Utc>,
    pub done_date: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,
}

impl Todo {
    pub fn to_domain_todo(self, tags: Vec<String>) -> TodoDomain {
        TodoDomain {
            id: Some(self.id),
            title: self.title,
            description: self.description,
            status: (self.status as i32).try_into().unwrap_or(Status::ABORTED),
            create_date: Some(self.create_date),
            done_date: self.done_date,
            deadline: self.deadline,
            tags,
        }
    }

    pub fn from_tuple( 
        tuple: (
            i32,
            String, 
            String, 
            Option<String>, 
            i32, 
            DateTime<Utc>, 
            Option<DateTime<Utc>>, 
            Option<DateTime<Utc>>
        )
    ) -> Self {
        Todo {
            id: tuple.0,
            username: tuple.1,
            title: tuple.2,
            description: tuple.3,
            status: tuple.4,
            create_date: tuple.5,
            done_date: tuple.6,
            deadline: tuple.7,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = _todo)]
pub struct NewTodo {
    pub username: String,
    pub title: String,
    pub description: Option<String>,
    pub status: i32,
    pub deadline: Option<DateTime<Utc>>,
}

impl NewTodo {
    pub fn from_domain_todo(todo: TodoDomain, username: &String) -> Self {
        NewTodo {
            username: username.to_owned(),
            title: todo.title,
            description: todo.description,
            status: todo.status as i32,
            deadline: todo.deadline,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = _todo_tag)]
pub struct NewTodoTag {
    pub todo_id: i32,
    pub tag_id: i32
}