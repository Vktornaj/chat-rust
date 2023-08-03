use async_trait::async_trait;
use rocket::futures::future::join_all;
use sqlx::query_builder::QueryBuilder;
use sqlx::{Postgres, Pool, Row};
use uuid::Uuid;

use crate::application::port::driven::user_repository::{UserRepositoryTrait, UpdateUser, NewUser, FindUser};
use crate::application::port::driven::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
    RepoUpdateError,
};
use crate::domain::user::User as UserDomain;
use super::models::user::User as UserDB;


pub struct UserRepository();

#[async_trait]
impl UserRepositoryTrait<Pool<Postgres>> for UserRepository {
    async fn find_by_id(&self, conn: &Pool<Postgres>, id: Uuid) -> Result<UserDomain, RepoSelectError> {
        let user = sqlx::query_as!(
            UserDB,
            r#"
                SELECT * FROM users WHERE id = $1
            "#,
            id
        ).fetch_one(conn).await;
        match user {
            Ok(user) => {
                let user_id = user.id;
                Ok(user.to_user_domain(get_languages(conn, &user_id).await.ok()))
            },
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err(RepoSelectError::NotFound),
                _ => Err(RepoSelectError::Unknown(err.to_string())),
            }
        }
    }

    // TODO: terminate this function
    async fn find_by_criteria(
        &self, 
        conn: &Pool<Postgres>,
        find_user: &FindUser,
        offset: i64,
        limit: i64,        
    ) -> Result<Vec<UserDomain>, RepoSelectError> {
        let mut query = QueryBuilder::new("SELECT * FROM users WHERE");
    
        if let Some(email) = &find_user.email {
            query.push(" email = ");
            query.push_bind(email);
        }
        if let Some(phone_number) = &find_user.phone_number {
            query.push(" phone_number = ");
            query.push_bind(phone_number);
        }
        if let Some(birthday) = &find_user.birthday {
            query.push(" birthday >= ");
            query.push_bind(birthday.0);
            query.push(" birthday < ");
            query.push_bind(birthday.1);
        }
        if let Some(nationality) = &find_user.nationality {
            query.push(" nationality = ");
            query.push_bind(nationality);
        }
        if let Some(languages) = &find_user.languages {
            query.push(" languages = ");
            query.push_bind(languages);
        }
        if let Some(created_at) = &find_user.created_at {
            query.push(" created_at >= ");
            query.push_bind(created_at.0);
            query.push(" created_at < ");
            query.push_bind(created_at.1);
        }
        if let Some(languages) = &find_user.languages {
            query.push(" languages = ");
            query.push_bind(languages);
        }

        // Execute the update query
        match query.build().fetch_all(conn).await {
            Ok(result) => {
                let users = result.iter().map(|x| UserDB {
                    id: x.get("id"),
                    email: x.get("email"),
                    phone_number: x.get("phone_number"),
                    password: x.get("password"),
                    first_name: x.get("first_name"),
                    last_name: x.get("last_name"),
                    birthday: x.get("birthday"),
                    nationality: x.get("nationality"),
                    created_at: x.get("created_at"),
                    updated_at: x.get("updated_at"),
                }).collect::<Vec<UserDB>>();
                let futures = users.iter()
                    .map(|x| get_languages(conn, &x.id));
                let every_languages = join_all(futures).await;
                Ok(users.into_iter().zip(every_languages)
                    .map(|(user, tags)| {
                        user.to_user_domain(tags.ok())
                }).collect())
            },
            Err(err) => return Err(RepoSelectError::Unknown(err.to_string())),
        }
    }

    async fn create(&self, conn: &Pool<Postgres>, user: NewUser) -> Result<UserDomain, RepoCreateError> {
        let result = sqlx::query!(
            r#"
                SELECT * FROM insert_user($1, $2, $3, $4, $5, $6, $7, $8);
            "#,
            user.email,
            user.phone_number,
            user.password,
            user.first_name,
            user.last_name,
            user.birthday,
            user.nationality,
            &user.languages
        ).fetch_one(conn).await;
        match result {
            Ok(result) => {
                let user = UserDB {
                    id: result.id.unwrap(),
                    email: result.email,
                    phone_number: result.phone_number,
                    password: result.password.unwrap(),
                    first_name: result.first_name,
                    last_name: result.last_name,
                    birthday: result.birthday.unwrap(),
                    nationality: result.nationality.unwrap(),
                    created_at: result.created_at.unwrap(),
                    updated_at: result.updated_at.unwrap()
                };
                Ok(user.to_user_domain(Some(get_languages(conn, &result.id.unwrap())
                    .await.unwrap_or(Vec::new()))))
            },
            Err(err) => Err(RepoCreateError::Unknown(err.to_string()))
        }
    }

    async fn update(&self, conn: &Pool<Postgres>, user: UpdateUser) -> Result<UserDomain, RepoUpdateError> {
        let mut query = QueryBuilder::new("UPDATE users SET");
    
        if let Some(email) = user.email {
            query.push(" email = ");
            query.push_bind(email);
        }
        if let Some(phone_number) = user.phone_number {
            query.push(" phone_number = ");
            query.push_bind(phone_number);
        }
        if let Some(password) = user.password {
            query.push(" password = ");
            query.push_bind(password);
        }
        if let Some(first_name) = user.first_name {
            query.push(" first_name = ");
            query.push_bind(first_name);
        }
        if let Some(last_name) = user.last_name {
            query.push(" last_name = ");
            query.push_bind(last_name);
        }
        if let Some(birthday) = user.birthday {
            query.push(" birthday = ");
            query.push_bind(birthday);
        }
        if let Some(nationality) = user.nationality {
            query.push(" nationality = ");
            query.push_bind(nationality);
        }

        // Add the WHERE clause with the user ID
        query.push(" WHERE id = ");
        query.push_bind(user.id);
    
        // Execute the update query
        match query.build().execute(conn).await {
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
            Err(err) => return Err(RepoUpdateError::Unknown(err.to_string())),
        }
    }

    async fn delete(&self, conn: &Pool<Postgres>, id: Uuid) -> Result<UserDomain, RepoDeleteError> {
        let result = sqlx::query_as!(
            UserDB,
            r#"
                DELETE FROM users WHERE id = $1 RETURNING 
                id, 
                email, 
                phone_number, 
                password, 
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
                    let languages = get_languages(conn, &user.id).await.unwrap_or(Vec::new());
                    return Ok(user.to_user_domain(Some(languages)));
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
            FROM users AS u
            JOIN users_languages AS ul ON ul.user_id = u.id
            JOIN languages AS l ON l.id = ul.language_id
            WHERE u.id = $1;
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