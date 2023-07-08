extern crate diesel;
use diesel::{prelude::*};
use async_trait::async_trait;

use crate::application::port::driven::user_repository;
use crate::domain::user::User as UserDomain;
use super::models::user::{User as UserDB, NewUser as NewUserDB};
use crate::application::port::driven::errors;
use super::db::Db;
use crate::adapter::driven::persistence::pgsql::schema;
use self::schema::_user::dsl::{
    username as _username, 
    _user,
    first_name as _first_name,
    last_name as _last_name
};


pub struct UserRepository {}

#[async_trait]
impl user_repository::UserRepository<Db> for UserRepository {
    async fn find_one(&self, conn: &Db, username: &String) -> Result<UserDomain, errors::RepoSelectError> {
        let username = username.to_owned();
        match conn.run(move |c| {
            _user.filter(_username.like(username)).first::<UserDB>(c)
        }).await {
            Ok(user_db) => Ok(user_db.to_user_domain()),
            Err(_) => Err(errors::RepoSelectError::NotFound)
        }
    }

    async fn create(&self, conn: &Db, user: UserDomain) -> Result<UserDomain, errors::RepoCreateError> {
        match conn.run(move |c| {
            diesel::insert_into(_user)
                .values(NewUserDB::from_user_domain(user))
                .get_result::<UserDB>(c)
        }).await {
            Ok(user_db) => Ok(user_db.to_user_domain()),
            Err(_) => Err(errors::RepoCreateError::Unknown("db error".to_owned()))
        }
    }

    async fn update(&self, conn: &Db, user: UserDomain) -> Result<UserDomain, errors::RepoUpdateError> {
        conn.run(move |c| {
            let mut res = Err(errors::RepoUpdateError::InvalidData("db error".to_owned()));
            if let Some(first_name) = user.first_name {
                res = match diesel::update(_user.filter(_username.eq(&user.username)))
                    .set(_first_name.eq(first_name))
                    .get_result::<UserDB>(c) 
                {
                    Ok(user_db) => Ok(user_db.to_user_domain()),
                    Err(_) => Err(errors::RepoUpdateError::Unknown("db error".to_owned()))
                };
            };
            if let Some(last_name) = user.last_name {
                res = match diesel::update(_user.filter(_username.eq(&user.username)))
                    .set(_last_name.eq(last_name))
                    .get_result::<UserDB>(c) 
                {
                    Ok(user_db) => Ok(user_db.to_user_domain()),
                    Err(_) => Err(errors::RepoUpdateError::Unknown("db error".to_owned()))
                };
            };
            res
        }).await
    }

    async fn delete(&self, conn: &Db, username: &String) -> Result<UserDomain, errors::RepoDeleteError> {
        let username = username.to_owned();
        match conn.run(move |c| {
            diesel::delete(_user.filter(_username.like(username)))
                .get_result::<UserDB>(c)
        }).await {
            Ok(user_db) => Ok(user_db.to_user_domain()),
            Err(_) => Err(errors::RepoDeleteError::Unknown("db error".to_owned()))
        }
    }
}