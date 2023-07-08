extern crate diesel;
use chrono::{DateTime, Utc, FixedOffset, TimeZone};
use diesel::result;
use diesel::pg::{PgConnection};
use diesel::{prelude::*};
use async_trait::async_trait;
use diesel::sql_types::{Nullable, Int4, Array, VarChar, Timestamptz};

use crate::application::port::driven::{todo_repository, errors};
use crate::domain::todo::Todo as TodoDomain;
use super::models::todo::{Todo as TodoDB, NewTodo as NewTodoDB, NewTodoTag};
use super::db::Db;
use crate::adapter::driven::persistence::pgsql::schema;
use self::schema::_todo::dsl::{
    _todo,
    username as _todo_username,
    id as _todo_id,
};
use self::schema::_todo_tag::dsl::{
    _todo_tag,
    todo_id as _todo_tag_todo_id,
};


// TODO: add "sql" suffix to all sql functions here and onto migrations
// Postgres functions
sql_function! { 
    fn find_todo_sql(
        p_username: VarChar, 
        p_title: Nullable<VarChar>, 
        p_description: Nullable<VarChar>, 
        p_status: Nullable<Int4>, 
        p_tags: Nullable<Array<VarChar>>,
    ) -> Record<(
        Int4, VarChar, VarChar, Nullable<VarChar>, Int4, Timestamptz, Nullable<Timestamptz>, Nullable<Timestamptz>,
    )>;
}

sql_function! { 
    fn find_tags_sql(p_todo_id: Int4) -> Nullable<Array<VarChar>>;
}

sql_function! { 
    fn create_tag(p_tag_value: VarChar, p_username: VarChar) -> Record<(Int4, VarChar)>;
}

sql_function! { 
    fn update_todo(
        p_id: Int4,
        p_title: Nullable<VarChar>, 
        p_description: Nullable<VarChar>, 
        p_status: Nullable<Int4>, 
        p_done_date: Nullable<Timestamptz>, 
        p_deadline: Nullable<Timestamptz>, 
    ) -> Record<(
        Int4, 
        VarChar, 
        VarChar, 
        Nullable<VarChar>, 
        Int4, 
        Timestamptz, 
        Nullable<Timestamptz>, 
        Nullable<Timestamptz>,
    )>;
}

pub struct TodoRepository {}

#[async_trait]
impl todo_repository::TodoRepository<Db> for TodoRepository {
    async fn find_one(&self, conn: &Db,  id: i32) -> Result<TodoDomain, errors::RepoSelectError> {
        match conn.run(move |c| {
            let res = _todo.find(id)
                .get_result::<TodoDB>(c);
            match res {
                Ok(
                    todo
                ) => match diesel::select(find_tags_sql(todo.id)).get_result::<Option<Vec<String>>>(c) {
                    Ok(tags) => Ok((todo, tags)),
                    Err(err) => {
                        println!("err: {}", err);
                        Err(errors::RepoSelectError::Unknown("Not found tags".to_owned()))
                    },
                },
                Err(err) => match err {
                    result::Error::NotFound => Err(errors::RepoSelectError::NotFound),
                    _ => Err(errors::RepoSelectError::Unknown("".to_owned())),
                },
            }
        }).await {
            Ok((
                todo, 
                tags
            )) => Ok(todo.to_domain_todo(tags.unwrap_or(Vec::new()))),
            Err(err) => Err(err)
        }
    }

    async fn find_all(
        &self, 
        conn: &Db, 
        username: &String, 
        offset: i32, 
        limit: i32
    ) -> Result<Vec<TodoDomain>, errors::RepoFindAllError> {
        let username = username.to_owned();
        match conn.run(move |c| {
            let res: Result<Vec<TodoDB>, result::Error> = _todo.filter(_todo_username.eq(username))
                .order(_todo_id)
                .limit(limit as i64)
                .offset(offset as i64)
                .load::<TodoDB>(c);
            match res {
                Ok(todos) => {
                    let res = todos.into_iter().map(|x| {
                        match diesel::select(find_tags_sql(x.id)).get_result::<Option<Vec<String>>>(c) {
                            Ok(tags) => Ok((x, tags)),
                            Err(err) => {
                                println!("err: {}", err);
                                Err(errors::RepoFindAllError::Unknown("Not found tags".to_owned()))
                            },
                        }
                    }).collect::<Vec<Result<(TodoDB, Option<Vec<String>>), errors::RepoFindAllError>>>();
                    Ok(res)
                },
                Err(err) => match err {
                    result::Error::NotFound => Err(errors::RepoFindAllError::Unknown("".to_owned())),
                    _ => Err(errors::RepoFindAllError::Unknown("".to_owned())),
                },
            }
        }).await {
            Ok(todos_tags_tuple) => {
                Ok(todos_tags_tuple.into_iter().map(|x| {
                    let (todo, tags) = x.unwrap();
                    todo.to_domain_todo(tags.unwrap_or(Vec::new()))
                }).collect())
            },
            Err(err) => Err(err)
        }
    }

    async fn find_one_criteria(
        &self, 
        conn: &Db, 
        username: &String, 
        find_todo: todo_repository::FindTodo
    ) -> Result<TodoDomain, errors::RepoSelectError> {
        match conn.run(move |c| {
            let res = diesel::select(find_todo_sql(
                find_todo.username,
                find_todo.title,
                find_todo.description,
                find_todo.status.and_then(|x| Some(x as i32)),
                find_todo.tags
            )).get_result::<(
                i32,
                String, 
                String, 
                Option<String>, 
                i32, 
                DateTime<Utc>, 
                Option<DateTime<Utc>>, 
                Option<DateTime<Utc>>
            )>(c);
            match res {
                Ok(
                    todo_tuple
                ) => match diesel::select(find_tags_sql(todo_tuple.0)).get_result::<Option<Vec<String>>>(c) {
                    Ok(tags) => Ok((todo_tuple, tags)),
                    Err(err) => {
                        println!("err: {}", err);
                        Err(errors::RepoSelectError::Unknown("Not found tags".to_owned()))
                    },
                },
                Err(err) => match err {
                    result::Error::NotFound => Err(errors::RepoSelectError::NotFound),
                    _ => Err(errors::RepoSelectError::Unknown("".to_owned())),
                },
            }
        }).await {
            Ok((
                todo_tuple, 
                tags
            )) => Ok(TodoDB::from_tuple(todo_tuple).to_domain_todo(tags.unwrap_or(Vec::new()))),
            Err(err) => Err(err)
        }
    }

    async fn find_all_criteria(
        &self, conn: &Db, 
        username: &String,
        from: i32, 
        to: i32, 
        find_todo: todo_repository::FindTodo
    ) -> Result<Vec<TodoDomain>, errors::RepoFindAllError> {
        match conn.run(move |c| {
            let res = diesel::select(find_todo_sql(
                find_todo.username,
                find_todo.title,
                find_todo.description,
                find_todo.status.and_then(|x| Some(x as i32)),
                find_todo.tags
            )).get_results::<(
                i32,
                String, 
                String, 
                Option<String>, 
                i32, 
                DateTime<Utc>, 
                Option<DateTime<Utc>>, 
                Option<DateTime<Utc>>
            )>(c);
            match res {
                Ok(todo_tuples) => {
                    let todo_tag_tuples: Vec<Result<TodoDomain, errors::RepoFindAllError>> = todo_tuples.into_iter().map(
                        |x| {
                            if let Ok(todo) = to_domain_todo(c, TodoDB::from_tuple(x)) {
                                Ok(todo)
                            } else {
                                Err(errors::RepoFindAllError::Unknown("".to_string()))
                            }
                        }
                    ).collect();
                    Ok(todo_tag_tuples)
                },
                Err(_) => Err(errors::RepoFindAllError::Unknown("".to_owned()))
            }
        }).await {
            Ok(todos) => {
               todos.into_iter().filter(|x| x.is_ok()).collect()
            },
            Err(err) => Err(err)
        }
    }

    async fn create(
        &self, 
        conn: &Db, 
        username: &String, 
        todo: TodoDomain
    ) -> Result<TodoDomain, errors::RepoCreateError> {
        let tags = todo.tags.clone();
        let tags_2 = todo.tags.clone();
        let username = username.to_owned();
        match conn.run(move |c| {
            let res: Result<TodoDB, result::Error> = diesel::insert_into(_todo)
                .values(NewTodoDB::from_domain_todo(todo, &username))
                .get_result::<TodoDB>(c);
            match res {
                Ok(todo) => {
                    for tag in tags.iter() {
                        let res = diesel::select(
                            create_tag(tag, &username)
                        ).get_result::<(i32, String)>(c);
                        if let Ok((tag_id, _)) = res {
                            if diesel::insert_into(_todo_tag)
                                .values(NewTodoTag { tag_id, todo_id: todo.id })
                                .execute(c).is_err() {
                                return Err(errors::RepoCreateError::Unknown("".to_owned()));
                            }
                        }; 
                        if let Err(err) = res {
                            println!("err: {}", err);
                            return Err(errors::RepoCreateError::Unknown("".to_owned()));
                        }
                    }
                    Ok(todo)
                },
                Err(_) => Err(errors::RepoCreateError::Unknown("".to_owned()))
            }
        }).await {
            Ok(todo) => Ok(todo.to_domain_todo(tags_2)),
            Err(_) => Err(errors::RepoCreateError::Unknown("db error".to_owned()))
        }
    }

    async fn update(
        &self, 
        conn: &Db, 
        todo: todo_repository::UpdateTodo
    ) -> Result<TodoDomain, errors::RepoUpdateError> {
        match conn.run(move |c| {
            // Placeholder to no change: '9999-12-31 23:59:59.999999+00'
            let date_str = "9999-12-31 23:59:59.999999";
            let format = "%Y-%m-%d %H:%M:%S%.6f";
            let placeholder: DateTime<Utc> = Utc.datetime_from_str(date_str, format).unwrap();
            let done_date: Option<DateTime<Utc>> = todo.done_date.unwrap_or(Some(placeholder));
            let deadline: Option<DateTime<Utc>> = todo.deadline.unwrap_or(Some(placeholder));

            let res = diesel::select(update_todo(
                todo.id,
                todo.title,
                todo.description,
                todo.status.and_then(|x| Some(x as i32)),
                done_date,
                deadline
            )).get_result::<(
                i32,
                String, 
                String, 
                Option<String>, 
                i32, 
                DateTime<Utc>, 
                Option<DateTime<Utc>>, 
                Option<DateTime<Utc>>
            )>(c);
            match res {
                Ok(
                    todo_tuple
                ) => match diesel::select(find_tags_sql(todo_tuple.0)).get_result::<Option<Vec<String>>>(c) {
                    Ok(tags) => Ok((todo_tuple, tags)),
                    Err(err) => {
                        println!("err: {}", err);
                        Err(errors::RepoUpdateError::Unknown("Not found tags".to_owned()))
                    },
                },
                Err(err) => {
                    println!("err: {:?}", &err);
                    match err {
                        result::Error::NotFound => Err(errors::RepoUpdateError::NotFound),
                        _ => Err(errors::RepoUpdateError::Unknown("".to_owned())),
                    }
                },
            }
        }).await {
            Ok((
                todo_tuple, 
                tags
            )) => Ok(TodoDB::from_tuple(todo_tuple).to_domain_todo(tags.unwrap_or(Vec::new()))),
            Err(err) => Err(err)
        }
    }

    async fn add_tag(
        &self, 
        conn: &Db, 
        todo_id: i32, 
        tag: &String
    ) -> Result<TodoDomain, errors::RepoUpdateError> {
        let tag = tag.to_owned();
        match conn.run(move |c| {
            let todo = if let Ok(todo) = _todo.find(todo_id).first::<TodoDB>(c) {
                todo
            } else {
                return Err(errors::RepoUpdateError::NotFound);
            };
            let tag_id = if let Ok(tag) = diesel::select(create_tag(tag, &todo.username))
                .get_result::<(i32, String)>(c) {
                tag.0
            } else {
                return Err(errors::RepoUpdateError::Unknown("".to_string()))
            };
            match diesel::insert_into(_todo_tag)
                .values(&NewTodoTag { tag_id, todo_id: todo_id })
                .execute(c) {
                Ok(_) => match diesel::select(find_tags_sql(todo.id)).get_result::<Option<Vec<String>>>(c) {
                    Ok(tags) => Ok((todo, tags)),
                    Err(err) => {
                        println!("err: {}", err);
                        Err(errors::RepoUpdateError::Unknown("Not found tags".to_owned()))
                    },
                },
                Err(err) => match err {
                    result::Error::NotFound => Err(errors::RepoUpdateError::NotFound),
                    _ => Err(errors::RepoUpdateError::Unknown("".to_owned())),
                },
            }
        }).await {
            Ok((
                todo, 
                tags
            )) => Ok(todo.to_domain_todo(tags.unwrap_or(Vec::new()))),
            Err(err) => Err(err)
        }
    }

    async fn remove_tag(
        &self, 
        conn: &Db, 
        todo_id: i32, 
        tag: &String
    ) -> Result<TodoDomain, errors::RepoUpdateError> {
        let tag = tag.to_owned();
        match conn.run(move |c| {
            let todo = if let Ok(todo) = _todo.find(todo_id).first::<TodoDB>(c) {
                todo
            } else {
                return Err(errors::RepoUpdateError::NotFound);
            };
            let tag_id = if let Ok(tag) = diesel::select(create_tag(tag, &todo.username))
                .get_result::<(i32, String)>(c) {
                tag.0
            } else {
                return Err(errors::RepoUpdateError::Unknown("".to_string()))
            };
            match diesel::delete(_todo_tag.find((todo.id, tag_id)))
                .execute(c) {
                Ok(_) => match diesel::select(find_tags_sql(todo.id)).get_result::<Option<Vec<String>>>(c) {
                    Ok(tags) => Ok((todo, tags)),
                    Err(err) => {
                        println!("err: {}", err);
                        Err(errors::RepoUpdateError::Unknown("Not found tags".to_owned()))
                    },
                },
                Err(err) => match err {
                    result::Error::NotFound => Err(errors::RepoUpdateError::NotFound),
                    _ => Err(errors::RepoUpdateError::Unknown("".to_owned())),
                },
            }
        }).await {
            Ok((
                todo, 
                tags
            )) => Ok(todo.to_domain_todo(tags.unwrap_or(Vec::new()))),
            Err(err) => Err(err)
        }
    }

    async fn delete(&self, conn: &Db, id: i32) -> Result<TodoDomain, errors::RepoDeleteError> {
        match conn.run(move |c| {
            if diesel::delete(_todo_tag.filter(_todo_tag_todo_id.eq(id)))
                .execute(c).is_err() {
                return Err(errors::RepoDeleteError::Unknown("".to_owned()))
            }
            let res = diesel::delete(_todo.find(id))
                .get_result::<TodoDB>(c);
            match res {
                Ok(
                    todo
                ) => match diesel::select(find_tags_sql(todo.id)).get_result::<Option<Vec<String>>>(c) {
                    Ok(tags) => Ok((todo, tags)),
                    Err(err) => {
                        println!("err: {}", err);
                        Err(errors::RepoDeleteError::Unknown("Not found tags".to_owned()))
                    },
                },
                Err(err) => match err {
                    result::Error::NotFound => Err(errors::RepoDeleteError::NotFound),
                    _ => Err(errors::RepoDeleteError::Unknown("".to_owned())),
                },
            }
        }).await {
            Ok((
                todo, 
                tags
            )) => Ok(todo.to_domain_todo(tags.unwrap_or(Vec::new()))),
            Err(err) => Err(err)
        }
    }

    async fn delete_all_criteria(
        &self, conn: &Db, 
        find_todo: todo_repository::FindTodo
    ) -> Result<Vec<TodoDomain>, errors::RepoDeleteError> {
        match conn.run(move |c| {
            let res = diesel::select(find_todo_sql(
                find_todo.username,
                find_todo.title,
                find_todo.description,
                find_todo.status.and_then(|x| Some(x as i32)),
                find_todo.tags
            )).get_results::<(
                i32,
                String, 
                String, 
                Option<String>, 
                i32, 
                DateTime<Utc>, 
                Option<DateTime<Utc>>, 
                Option<DateTime<Utc>>
            )>(c);
            match res {
                Ok(todo_tuples) => {
                    let todo_tag_tuples: Vec<Result<TodoDomain, errors::RepoDeleteError>> = todo_tuples.into_iter().map(
                        |x| {
                            if let Ok(todo) = to_domain_todo(c, TodoDB::from_tuple(x)) {
                                Ok(todo)
                            } else {
                                Err(errors::RepoDeleteError::Unknown("".to_string()))
                            }
                        }
                    ).collect();
                    Ok(todo_tag_tuples)
                },
                Err(_) => Err(errors::RepoDeleteError::Unknown("".to_owned()))
            }
        }).await {
            Ok(todos) => {
               let res: Vec<TodoDomain> = todos.into_iter()
                .filter(|x| x.is_ok())
                .map(|x| x.unwrap())
                .collect();
                for todo in res.iter() {
                    self.delete(conn, todo.id.unwrap()).await?;
                };
                Ok(res)
            },
            Err(err) => Err(err)
        }
    }
}

fn to_domain_todo(conn: &mut PgConnection, todo: TodoDB) -> Result<TodoDomain, errors::RepoSelectError> {
    match diesel::select(find_tags_sql(todo.id)).get_result::<Option<Vec<String>>>(conn) {
        Ok(tags) => {
            Ok(todo.to_domain_todo(tags.unwrap_or(Vec::new())))
        },
        Err(err) => {
            println!("err: {}", err);
            Err(errors::RepoSelectError::Unknown("Not found tags".to_owned()))
        }
    }
}