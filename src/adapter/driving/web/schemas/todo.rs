use chrono::{Utc, TimeZone};
use serde::{Serialize, Deserialize};

use crate::{
    domain::todo::{Todo as TodoDomain, Status},
    config::DATE_FORMAT, 
    application::port::driven::todo_repository::UpdateTodo
};


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TodoJson {
    pub id: Option<i32>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
    pub create_date: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub done_date: Option<Option<String>>,
    #[serde(
        default,                                   
        skip_serializing_if = "Option::is_none",
        with = "::serde_with::rust::double_option",
    )]
    pub deadline: Option<Option<String>>,
    pub tags: Option<Vec<String>>,
}

impl TodoJson {
    pub fn new() -> Self {
        TodoJson { 
            id: None, 
            title: None, 
            description: None, 
            status: None, 
            create_date: None, 
            done_date: None, 
            deadline: None, 
            tags: None 
        }
    }
    // TODO: Fix this
    pub fn from_domain_todo(todo: TodoDomain) -> Self {
        TodoJson { 
            id: todo.id, 
            title: Some(todo.title), 
            description: todo.description, 
            status: Some(todo.status), 
            create_date: todo.create_date
                .and_then(|x| Some(x.format(DATE_FORMAT).to_string())), 
            done_date: Some(todo.done_date
                .and_then(|x| Some(x.format(DATE_FORMAT).to_string()))), 
            deadline: Some(todo.deadline
                .and_then(|x| Some(x.format(DATE_FORMAT).to_string()))), 
            tags: Some(todo.tags) 
        }
    }

    // TODO: Fix this
    pub fn to_domain_todo(self) -> TodoDomain {
        TodoDomain {
            id: None,
            title: self.title.unwrap(),
            description: self.description,
            status: self.status.unwrap_or(Status::PENDING),
            create_date: None,
            done_date: self.done_date
                .and_then(|x| x
                    .and_then(|x| Utc.datetime_from_str(&x, DATE_FORMAT).ok())),
            deadline: self.deadline
                .and_then(|x| x
                    .and_then(|x| Utc.datetime_from_str(&x, DATE_FORMAT).ok())),
            tags: self.tags.unwrap_or(Vec::new()),
        }
    }

    // TODO: Fix this
    pub fn to_update_todo(self) -> UpdateTodo {
        UpdateTodo {
            id: self.id.unwrap(),
            title: self.title, 
            description: self.description, 
            status: self.status, 
            done_date: self.done_date
                .and_then(|x| x
                    .and_then(|x| Some(Utc.datetime_from_str(&x, DATE_FORMAT).ok()))), 
            deadline: self.deadline
                .and_then(|x| x
                    .and_then(|x| Some(Utc.datetime_from_str(&x, DATE_FORMAT).ok()))), 
        }
    } 
}