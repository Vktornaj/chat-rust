use async_trait::async_trait;
use sqlx::{Postgres, Pool};

use crate::application::port::driven::user_repository::UserRepositoryTrait;
use crate::application::port::driven::errors::{
    RepoCreateError, 
    RepoDeleteError, 
    RepoSelectError, 
    RepoUpdateError
};
use crate::domain::user::User as UserDomain;
use crate::adapter::driven::persistence::sqlx::models::user::User as UserDB;


pub struct UserRepository {}

#[async_trait]
impl UserRepositoryTrait<Pool<Postgres>> for UserRepository {
    async fn find_one(
        &self, 
        conn: &Pool<Postgres>, 
        email: &Option<String>,
        phone_number: &Option<String>
    ) -> Result<UserDomain, RepoSelectError> {
        if email.is_none() && phone_number.is_none() {
            return Err(RepoSelectError::NotFound);
        }
        let user = sqlx::query_as!(
            UserDB,
            r#"
                SELECT * FROM users WHERE email = $1 OR phone_number = $2
            "#,
            email.unwrap_or("None".to_string()),
            phone_number.unwrap_or("None".to_string())
        ).fetch_one(conn).await;
        if let Err(err) = user {
            return match err {
                sqlx::Error::RowNotFound => Err(RepoSelectError::NotFound),
                _ => Err(RepoSelectError::Unknown(err.to_string())),
            }
        }
        let user = user.unwrap();
        let languages = sqlx::query!(
            r#"
                SELECT l.id
                FROM users AS u
                JOIN users_languages AS ul ON ul.user_id = u.id
                JOIN languages AS l ON l.id = ul.language_id
                WHERE u.id = $1;
            "#,
            user.id
        ).fetch_all(conn).await;
        match languages {
            Ok(languages) => Ok(user.to_user_domain(Some(languages.iter()
                .map(|l| l.id.to_string()).collect()))),
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err(RepoSelectError::NotFound),
                _ => Err(RepoSelectError::Unknown(err.to_string())),
            }
        }
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
                Ok(user.to_user_domain(None))
            },
            Err(err) => Err(RepoCreateError::Unknown(err.to_string()))
        }
    }

    async fn update(&self, conn: &Pool<Postgres>, user: UserDomain) -> Result<UserDomain, RepoUpdateError> {
        todo!()
    }

    async fn delete(&self, conn: &Pool<Postgres>, username: &String) -> Result<UserDomain, RepoDeleteError> {
        todo!()
    }
}