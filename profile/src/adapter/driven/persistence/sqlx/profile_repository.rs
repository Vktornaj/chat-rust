use async_trait::async_trait;
use chrono::NaiveDate;
use common::domain::types::error::ErrorMsg;
use sqlx::query_builder::QueryBuilder;
use sqlx::{Postgres, Pool};
use uuid::Uuid;
use futures::future::join_all;

use crate::application::port::driven::user_repository::{ProfileRepositoryTrait, UpdateUser, FindUser};
use crate::application::port::driven::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
    RepoUpdateError,
};
use crate::domain::profile::{Profile as UserDomain, NewProfile};
use super::models::profile::Profile as UserDB;


pub struct ProfileRepository();

#[async_trait]
impl ProfileRepositoryTrait<Pool<Postgres>> for ProfileRepository {
    async fn find_by_id(&self, conn: &Pool<Postgres>, id: Uuid) -> Result<UserDomain, RepoSelectError> {
        let profile = sqlx::query_as!(
            UserDB,
            r#"
                SELECT * FROM profiles WHERE user_id = $1
            "#,
            id
        ).fetch_one(conn).await;
        match profile {
            Ok(profile) => {
                let user_id = profile.user_id;
                let languages = if let Ok(languages) = get_languages(conn, &user_id).await {
                    languages
                } else {
                    return Err(RepoSelectError::Unknown("Error getting languages".to_string()));
                };
                match profile.to_user_domain(languages) {
                    Ok(user) => Ok(user),
                    Err(err) => Err(RepoSelectError::Unknown(err.to_string())),
                }
            },
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err(RepoSelectError::NotFound),
                _ => Err(RepoSelectError::Unknown(err.to_string())),
            }
        }
    }

    // TODO: test this function
    async fn find_by_criteria(
        &self, 
        conn: &Pool<Postgres>,
        find_user: FindUser,
        offset: i64,
        limit: i64,        
    ) -> Result<Vec<UserDomain>, RepoSelectError> {
        let mut query = QueryBuilder::new("SELECT users.* FROM profiles");
    
        if find_user.languages.is_some() {
            query.push(" INNER JOIN users_languages ON users.id = users_languages.user_id ");
            query.push(" INNER JOIN languages ON users_languages.language_id = languages.id ");
        }
        query.push(" WHERE TRUE ");
        if let Some(languages) = find_user.languages {
            let languages = languages.into_iter()
                .map(|x| Into::<String>::into(x)).collect::<Vec<String>>();
            query.push(" AND languages.code = ANY(");
            query.push_bind(languages);
            query.push(")");
        }
        if let Some(birthday) = &find_user.birthday {
            query.push(" AND birthday >= ");
            query.push_bind(birthday.0);
            query.push(" AND birthday < ");
            query.push_bind(birthday.1);
        }
        if let Some(nationality) = find_user.nationality {
            query.push(" AND nationality = ");
            query.push_bind(Into::<String>::into(nationality));
        }
        if let Some(created_at) = &find_user.created_at {
            query.push(" AND created_at >= ");
            query.push_bind(created_at.0);
            query.push(" AND created_at < ");
            query.push_bind(created_at.1);
        }
        query.push(" OFFSET ");
        query.push_bind(offset);
        query.push(" LIMIT ");
        query.push_bind(limit);

        // Execute the update query
        match query.build().fetch_all(conn).await {
            Ok(result) => {
                let res: Result<Vec<UserDB>, sqlx::Error> = result.iter()
                    .map(|x| UserDB::from_pgrow(x))
                    .collect();

                let users = if let Ok(users) = res {
                    users
                } else {
                    return Err(RepoSelectError::Unknown("Error getting users".to_string()));
                };

                let futures = users.iter().map(|x| get_languages(conn, &x.user_id));

                // run futures all at once
                let every_languages = join_all(futures)
                    .await
                    .into_iter()
                    .collect::<Result<Vec<Vec<String>>, RepoSelectError>>()?;
                
                let users: Result<Vec<UserDomain>, ErrorMsg> = users.into_iter().zip(every_languages)
                    .map(|(user, languages)| {
                        user.to_user_domain(languages)
                }).collect();

                match users {
                    Ok(users) => Ok(users),
                    Err(err) => Err(RepoSelectError::Unknown(err.to_string())),
                }
            },
            Err(err) => {
                println!("{}", err.to_string());
                Err(RepoSelectError::Unknown(err.to_string()))
            },
        }
    }

    async fn create(&self, conn: &Pool<Postgres>, new_user: NewProfile) -> Result<UserDomain, RepoCreateError> {
        let result = sqlx::query!(
            r#"
                SELECT * FROM insert_profile($1, $2, $3, $4, $5, $6);
            "#,
            Into::<Uuid>::into(new_user.user_id),
            Into::<String>::into(new_user.first_name),
            Into::<String>::into(new_user.last_name),
            Into::<NaiveDate>::into(new_user.birthday),
            Into::<String>::into(new_user.nationality),
            &new_user.languages.into_iter().map(|x| x.into()).collect::<Vec<String>>()
        ).fetch_one(conn).await;
        match result {
            Ok(result) => {
                let user = UserDB {
                    user_id: result.id.unwrap(),
                    first_name: result.first_name.unwrap(),
                    last_name: result.last_name.unwrap(),
                    birthday: result.birthday.unwrap(),
                    nationality: result.nationality.unwrap(),
                    created_at: result.created_at.unwrap(),
                    updated_at: result.updated_at.unwrap()
                };
                let user = if let Ok(languages) = get_languages(conn, &result.id.unwrap()).await {
                    match user.to_user_domain(languages) {
                        Ok(user) => user,
                        Err(err) => return Err(RepoCreateError::Unknown(err.to_string()))
                    }
                } else {
                    return Err(RepoCreateError::Unknown("Error getting user".to_string()));
                };
                Ok(user)
            },
            Err(err) => Err(RepoCreateError::Unknown(err.to_string()))
        }
    }

    async fn update(&self, conn: &Pool<Postgres>, user: UpdateUser) -> Result<UserDomain, RepoUpdateError> {
        let mut query_builder = QueryBuilder::new("UPDATE users SET ");
        let mut separated = query_builder.separated(", ");
    
        if let Some(first_name) = user.first_name {
            separated.push("first_name = ");
            separated.push_bind_unseparated(Into::<String>::into(first_name));
        }
        if let Some(last_name) = user.last_name {
            separated.push("last_name = ");
            separated.push_bind_unseparated(Into::<String>::into(last_name));
        }
        if let Some(birthday) = user.birthday {
            separated.push("birthday = ");
            separated.push_bind_unseparated(Into::<NaiveDate>::into(birthday));
        }
        if let Some(nationality) = user.nationality {
            separated.push("nationality = ");
            separated.push_bind_unseparated(Into::<String>::into(nationality));
        }

        // Add the WHERE clause with the user ID
        separated.push_unseparated(" WHERE id =");
        separated.push_bind_unseparated(user.id);
    
        // Execute the update query
        match query_builder.build().execute(conn).await {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    match self.find_by_id(conn, user.id).await {
                        Ok(updated_user) => return Ok(updated_user),
                        Err(_) => return Err(RepoUpdateError::Unknown("".to_string())),
                    }
                } else {
                    Err(RepoUpdateError::NotFound)
                }
            },
            Err(err) => return Err(
                RepoUpdateError::Unknown(
                    format!("Error updating user {} {}", err.to_string(), query_builder.into_sql())
                )
            ),
        }
    }

    async fn delete(&self, conn: &Pool<Postgres>, id: Uuid) -> Result<UserDomain, RepoDeleteError> {
        let result = sqlx::query_as!(
            UserDB,
            r#"
                DELETE FROM profiles WHERE user_id = $1 RETURNING 
                user_id, 
                first_name, 
                last_name, 
                birthday, 
                nationality, 
                created_at, 
                updated_at
            "#,
            id
        ).fetch_optional(conn).await;
        match result {
            Ok(result) => {
                if let Some(user) = result {
                    match get_languages(conn, &user.user_id).await {
                        Ok(languages) => match user.to_user_domain(languages) {
                            Ok(user) => return Ok(user),
                            Err(err) => return Err(RepoDeleteError::Unknown(err.to_string())),
                        },
                        Err(_) => return Err(RepoDeleteError::Unknown("Error getting languages".to_string())),
                    }
                } else {
                    return Err(RepoDeleteError::NotFound);
                }
            },
            Err(err) => return Err(RepoDeleteError::Unknown(err.to_string())),
        }
    }
}

async fn get_languages(conn: &Pool<Postgres>, user_id: &Uuid) -> Result<Vec<String>, RepoSelectError> { 
    let languages = sqlx::query!(
        r#"
            SELECT l.code
            FROM profiles AS u
            JOIN profiles_languages AS ul ON ul.user_id = u.user_id
            JOIN languages AS l ON l.id = ul.language_id
            WHERE u.user_id = $1;
        "#,
        user_id
    ).fetch_all(conn).await;
    match languages {
        Ok(languages) => Ok(languages.iter()
            .map(|l| l.code.to_owned()).collect()),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(RepoSelectError::NotFound),
            _ => Err(RepoSelectError::Unknown(err.to_string())),
        }
    }
}
