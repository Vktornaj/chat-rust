extern crate rocket;
use rocket::http::Status;
use rocket::{get, post, put, delete, State};
use rocket::serde::{json::Json};

use crate::adapter::driven::persistence::pgsql::db::Db;
use crate::adapter::driving::web::{
    schemas::todo::TodoJson,
    token::Token
};
use crate::application::use_cases::{self, todo_add_tag, todo_remove_tag};
use crate::application::use_cases::delete_todo::DeleteError;
use crate::application::use_cases::find_todos::FindAllError;
use crate::application::use_cases::{
    create_todo::CreateError,
    update_todo::UpdateError,
};
use crate::config::AppState;

// Persistence
use crate::adapter::driven::persistence::pgsql::todo_repository::TodoRepository;


#[post("/todo", format = "json", data = "<todo>")]
pub async fn post_todo(
    connection: Db,
    state: &State<AppState>, 
    token: Token,
    todo: Json<TodoJson>
) -> Result<Json<TodoJson>, Status> {
    match use_cases::create_todo::execute(
        &connection, 
        &TodoRepository {}, 
        &state.secret,
        &token.value,
        todo.0.to_domain_todo()
    ).await {
        Ok(todo) => Ok(Json(TodoJson::from_domain_todo(todo))),
        Err(err) => match err {
            CreateError::Conflict(_) => Err(Status::Conflict),
            CreateError::InvalidData(_) => Err(Status::BadRequest),
            CreateError::Unknown(_) => Err(Status::InternalServerError),
            CreateError::Unautorized(_) => Err(Status::Unauthorized),
        },
    }
}

#[put("/todo", format = "json", data = "<todo>")]
pub async fn update_todo(
    connection: Db,
    state: &State<AppState>, 
    token: Token,
    todo: Json<TodoJson>
) -> Result<Json<TodoJson>, Status> {
    match use_cases::update_todo::execute(
        &connection, 
        &TodoRepository {}, 
        &state.secret, 
        &token.value, 
        todo.0.to_update_todo()
    ).await {
        Ok(todo) => Ok(Json(TodoJson::from_domain_todo(todo))),
        Err(err) => match err {
            UpdateError::InvalidData(_) => Err(Status::BadRequest),
            UpdateError::Unknown(_) => Err(Status::InternalServerError),
            UpdateError::Unautorized(_) => Err(Status::Unauthorized),
        },
    }
}

#[delete("/todo/<id>")]
pub async fn delete_todo(
    connection: Db,
    state: &State<AppState>, 
    token: Token,
    id: i32
) -> Result<Json<TodoJson>, Status> {
    match use_cases::delete_todo::execute(
        &connection,
        &TodoRepository {}, 
        &state.secret, 
        &token.value, 
        id.into()
    ).await {
        Ok(todo) => Ok(Json(TodoJson::from_domain_todo(todo))),
        Err(err) => match err {
            DeleteError::InvalidData(_) => Err(Status::BadRequest),
            DeleteError::Unknown(_) => Err(Status::InternalServerError),
            DeleteError::Unautorized(_) => Err(Status::Unauthorized),
            DeleteError::Conflict(_) => Err(Status::Conflict),
        },
    }
}

#[get("/todos/<from>/<to>")]
pub async fn get_todos(
    connection: Db,
    state: &State<AppState>, 
    token: Token,
    from: i32, 
    to: i32
) -> Result<Json<Vec<TodoJson>>, Status> {
    match use_cases::find_todos::execute(
        &connection,
        &TodoRepository {}, 
        &state.secret, 
        &token.value, 
        from,
        to
    ).await {
        Ok(todos) => Ok(Json(todos.into_iter()
            .map(|x| TodoJson::from_domain_todo(x)).collect())),
        Err(err) => match err {
            FindAllError::Unknown(_) => Err(Status::InternalServerError),
            FindAllError::Unautorized(_) => Err(Status::Unauthorized),
        },
    }
}

#[put("/todo/<id>/tag/<tag>")]
pub async fn put_add_tag(
    connection: Db,
    state: &State<AppState>, 
    token: Token,
    id: i32, 
    tag: String
) -> Result<Json<TodoJson>, Status> {
    match use_cases::todo_add_tag::execute(
        &connection,
        &TodoRepository {}, 
        &state.secret, 
        &token.value, 
        id,
        &tag
    ).await {
        Ok(todo) => Ok(Json(TodoJson::from_domain_todo(todo))),
        Err(err) => match err {
            todo_add_tag::UpdateError::Unknown(_) => Err(Status::InternalServerError),
            todo_add_tag::UpdateError::Unautorized(_) => Err(Status::Unauthorized),
            todo_add_tag::UpdateError::InvalidData(_) => Err(Status::NoContent),
            todo_add_tag::UpdateError::Conflict(_) => Err(Status::NotAcceptable),
        },
    }
}

#[delete("/todo/<id>/tag/<tag>")]
pub async fn put_remove_tag(
    connection: Db,
    state: &State<AppState>, 
    token: Token,
    id: i32, 
    tag: String
) -> Result<Json<TodoJson>, Status> {
    match use_cases::todo_remove_tag::execute(
        &connection,
        &TodoRepository {}, 
        &state.secret, 
        &token.value, 
        id,
        &tag
    ).await {
        Ok(todo) => Ok(Json(TodoJson::from_domain_todo(todo))),
        Err(err) => match err {
            todo_remove_tag::UpdateError::Unknown(_) => Err(Status::InternalServerError),
            todo_remove_tag::UpdateError::Unautorized(_) => Err(Status::Unauthorized),
            todo_remove_tag::UpdateError::InvalidData(_) => Err(Status::NoContent),
            todo_remove_tag::UpdateError::Conflict(_) => Err(Status::NotAcceptable),
        },
    }
}