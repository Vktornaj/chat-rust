use async_trait::async_trait;
use sqlx::query_builder::QueryBuilder;
use sqlx::{Postgres, Pool};

use crate::application::port::driven::user_repository::{UserRepositoryTrait, UpdateUser};
use crate::application::port::driven::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
    RepoUpdateError
};
use crate::domain::user::User as UserDomain;
use crate::adapter::driven::persistence::sqlx::models::user::User as UserDB;


pub struct UserRepository();

#[async_trait]
impl UserRepositoryTrait<Pool<Postgres>> for UserRepository {
    async fn find_by_id(&self, conn: &Pool<Postgres>, id: i32) -> Result<UserDomain, RepoSelectError> {
        let user = sqlx::query_as!(
            UserDB,
            r#"
                SELECT * FROM users WHERE id = $1
            "#,
            id
        ).fetch_one(conn).await;
        if let Err(err) = user {
            return match err {
                sqlx::Error::RowNotFound => Err(RepoSelectError::NotFound),
                _ => Err(RepoSelectError::Unknown(err.to_string())),
            }
        }
        let user = user.unwrap();
        Ok(user.to_user_domain(Some(get_languages(conn, user.id)
            .await.unwrap_or(Vec::new()))))
    }

    async fn find_one_by_email(
        &self, 
        conn: &Pool<Postgres>, 
        email: &String
    ) -> Result<UserDomain, RepoSelectError> {
        let user = sqlx::query_as!(
            UserDB,
            r#"
                SELECT * FROM users WHERE email = $1
            "#,
            email,
        ).fetch_one(conn).await;
        if let Err(err) = user {
            return match err {
                sqlx::Error::RowNotFound => Err(RepoSelectError::NotFound),
                _ => Err(RepoSelectError::Unknown(err.to_string())),
            }
        }
        let user = user.unwrap();
        Ok(user.to_user_domain(Some(get_languages(conn, user.id)
            .await.unwrap_or(Vec::new()))))
    }
    
    async fn find_one_by_phone_number(
        &self, 
        conn: &Pool<Postgres>, 
        phone_number: &String
    ) -> Result<UserDomain, RepoSelectError> {
        let user = sqlx::query_as!(
            UserDB,
            r#"
                SELECT * FROM users WHERE phone_number = $1
            "#,
            phone_number
        ).fetch_one(conn).await;
        if let Err(err) = user {
            return match err {
                sqlx::Error::RowNotFound => Err(RepoSelectError::NotFound),
                _ => Err(RepoSelectError::Unknown(err.to_string())),
            }
        }
        let user = user.unwrap();
        Ok(user.to_user_domain(Some(get_languages(conn, user.id)
            .await.unwrap_or(Vec::new()))))
    }

    async fn create(&self, conn: &Pool<Postgres>, user: UserDomain) -> Result<UserDomain, RepoCreateError> {
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
            &user.languages.unwrap_or(vec![]),
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
                Ok(user.to_user_domain(Some(get_languages(conn, result.id.unwrap())
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

    async fn delete(&self, conn: &Pool<Postgres>, id: i32) -> Result<UserDomain, RepoDeleteError> {
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
                    return Ok(user
                        .to_user_domain(Some(get_languages(conn, user.id)
                        .await.unwrap_or(vec![]))));
                } else {
                    return Err(RepoDeleteError::NotFound);
                }
            },
            Err(err) => return Err(RepoDeleteError::Unknown(err.to_string())),
        }
    }
}

async fn get_languages(conn: &Pool<Postgres>, user_id: i32) -> Result<Vec<String>, RepoSelectError> {
    let languages = sqlx::query!(
        r#"
            SELECT l.id
            FROM users AS u
            JOIN users_languages AS ul ON ul.user_id = u.id
            JOIN languages AS l ON l.id = ul.language_id
            WHERE u.id = $1;
        "#,
        user_id
    ).fetch_all(conn).await;
    match languages {
        Ok(languages) => Ok(languages.iter()
            .map(|l| l.id.to_string()).collect()),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err(RepoSelectError::NotFound),
            _ => Err(RepoSelectError::Unknown(err.to_string())),
        }
    }
}