use async_trait::async_trait;
use sqlx::{Postgres, Pool};
use uuid::Uuid;

use crate::application::port::driven::auth_repository::{AuthRepositoryTrait, UpdateIdentify};
use crate::domain::auth::{Auth, NewAuth};
use crate::domain::types::identification::{IdentificationValue, NewIdentification};
use super::models::auth::{AuthSQL, IdentificationSQL, TokenMetadataSQL};


pub struct AuthRepository();

#[async_trait]
impl AuthRepositoryTrait<Pool<Postgres>> for AuthRepository {
    async fn find_by_id(&self, conn: &Pool<Postgres>, user_id: Uuid) -> Result<Auth, String> {
        let auth = sqlx::query_as!(
            AuthSQL,
            r#"
                SELECT * FROM auths WHERE user_id = $1
            "#,
            user_id
        ).fetch_one(conn).await;
        match auth {
            Ok(auth) => {
                let user_id = auth.user_id;
                let identifications = if let Ok(identifications) = get_identifications(
                    conn, 
                    &user_id
                ).await {
                    identifications
                } else {
                    return Err("Error getting identifications".to_string());
                };
                let tokens_metadata = if let Ok(tokens_metadata) = get_tokens_metadata(
                    conn, 
                    &user_id
                ).await {
                    tokens_metadata
                } else {
                    return Err("Error getting tokens metadata".to_string());
                };
                match auth.to_auth_domain(identifications, tokens_metadata) {
                    Ok(user) => Ok(user),
                    Err(err) => Err(err.to_string()),
                }
            },
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err("User not found".to_string()),
                _ => Err(err.to_string()),
            }
        }
    }

    // TODO: test this function
    async fn find_by_identification(
        &self, 
        conn: &Pool<Postgres>,
        identification_value: IdentificationValue,
    ) -> Result<Vec<Auth>, String> {
        let identification_value: String = match identification_value {
            IdentificationValue::Email(email) => email.into(),
            IdentificationValue::PhoneNumber(phone_number) => phone_number.into(),
        };
        let auth_sql = sqlx::query_as!(
            AuthSQL,
            r#"
                SELECT a.* 
                FROM auths a JOIN identifications i 
                ON a.user_id = i.user_id 
                WHERE i.identification_value = $1;
            "#,
            identification_value
        ).fetch_one(conn).await;

        let identifications = match auth_sql {
            Ok(auth) => if let Ok(identifications) = get_identifications(
                conn, 
                &auth.user_id
            ).await {
                identifications
            } else {
                return Err("Error getting identifications".to_string());
            },
            Err(err) => match err {
                sqlx::Error::RowNotFound => return Err("User not found".to_string()),
                _ => return Err(err.to_string()),
            }
        };

        let tokens_metadata = match auth_sql {
            Ok(auth) => if let Ok(tokens_metadata) = get_tokens_metadata(
                conn, 
                &auth.user_id
            ).await {
                tokens_metadata
            } else {
                return Err("Error getting tokens metadata".to_string());
            },
            Err(err) => match err {
                sqlx::Error::RowNotFound => return Err("User not found".to_string()),
                _ => return Err(err.to_string()),
            }
        };

        match auth_sql {
            Ok(auth) => auth.to_auth_domain(identifications, tokens_metadata).into(),
            Err(err) => match err {
                sqlx::Error::RowNotFound => return Err("User not found".to_string()),
                _ => return Err(err.to_string()),
            }
        }
    }

    async fn create(
        &self, 
        conn: &Pool<Postgres>, 
        auth: NewAuth, 
        new_identification: NewIdentification
    ) -> Result<Auth, String> {
        let res_auth = sqlx::query_as!(
            AuthSQL,
            r#"
                INSERT INTO auths (hashed_password) VALUES ($1) 
                RETURNING *;
            "#,
            Into::<String>::into(auth.hashed_password),
        ).fetch_one(conn).await;

        let auth_sql = match res_auth {
            Ok(res_auth) => res_auth,
            Err(err) => return Err(err.to_string()),
        };

        let res_identification_id = sqlx::query_as!(
            IdentificationSQL,
            r#"
                INSERT INTO identifications (user_id, identification_type, identification_value) 
                VALUES ($1, $2, $3) RETURNING *;
            "#,
            auth_sql.user_id,
            new_identification.identification_value.get_type(),
            new_identification.identification_value,
        ).fetch_one(conn).await;

        let identification = match res_identification_id {
            Ok(res_identification_id) => res_identification_id,
            Err(err) => {
                sqlx::query!(
                    r#"
                        DELETE FROM auths WHERE user_id = $1;
                    "#,
                    auth_sql.user_id,
                ).execute(conn).await;
                return Err(err.to_string());
            },
        };

        auth_sql.to_auth_domain(vec![identification], vec!()).into()
        
    }

    async fn update_password(
        &self, 
        conn: &Pool<Postgres>, 
        user_id: Uuid,
        new_hashed_password: String,
    ) -> Result<Auth, String> {
        let auth = sqlx::query_as!(
            AuthSQL,
            r#"
                UPDATE auths SET hashed_password = $1 WHERE user_id = $2 RETURNING *;
            "#,
            new_hashed_password,
            user_id,
        ).fetch_one(conn).await;
        match auth {
            Ok(auth) => {
                let user_id = auth.user_id;
                let identifications = if let Ok(identifications) = get_identifications(
                    conn, 
                    &user_id
                ).await {
                    identifications
                } else {
                    return Err("Error getting identifications".to_string());
                };
                let tokens_metadata = if let Ok(tokens_metadata) = get_tokens_metadata(
                    conn, 
                    &user_id
                ).await {
                    tokens_metadata
                } else {
                    return Err("Error getting tokens metadata".to_string());
                };
                match auth.to_auth_domain(identifications, tokens_metadata) {
                    Ok(user) => Ok(user),
                    Err(err) => Err(err.to_string()),
                }
            },
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err("User not found".to_string()),
                _ => Err(err.to_string()),
            }
        }
    }

    async fn update_identifications(
        conn: &Pool<Postgres>, 
        identification_operation: UpdateIdentify<NewIdentification, Uuid>
    ) {
        let res_user_id = match identification_operation {
            UpdateIdentify::Add(new_identification) => {
                sqlx::query!(
                    r#"
                        INSERT INTO identifications (user_id, identification_type, identification_value) 
                        VALUES ($1, $2, $3) RETURNING user_id;
                    "#,
                    new_identification.user_id,
                    new_identification.identification_value.get_type(),
                    new_identification.identification_value,
                ).fetch_one(conn).await
            },
            UpdateIdentify::Delete(identification_id) => {
                sqlx::query!(
                    r#"
                        DELETE FROM identifications WHERE id = $1 RETURNING user_id;
                    "#,
                    identification_id,
                ).execute(conn).await
            },
        };

        let user_id = match res_user_id {
            Ok(res_user_id) => res_user_id.user_id,
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err("User not found".to_string()),
                _ => Err(err.to_string()),
            }
        };

        let res_auth = sqlx::query_as!(
            AuthSQL,
            r#"
                SELECT * FROM auths WHERE user_id = $1;
            "#,
            user_id
        ).fetch_one(conn).await;

        match res_auth {
            Ok(auth) => Ok(auth),
            Err(err) => match err {
                sqlx::Error::RowNotFound => Err("User not found".to_string()),
                _ => Err(err.to_string()),
            }
        }
    }

    async fn delete(&self, conn: &Pool<Postgres>, user_id: Uuid) -> Result<Auth, String> {

        let res_identifications = sqlx::query!(
            r#"
                DELETE FROM identifications WHERE user_id = $1 RETURNING *;
            "#,
            user_id
        ).fetch_all(conn).await;

        let res_tokens_metadata = sqlx::query!(
            r#"
                DELETE FROM tokens_metadata WHERE user_id = $1 RETURNING *;
            "#,
            user_id
        ).fetch_all(conn).await;

        let res_auth_sql = sqlx::query_as!(
            AuthSQL,
            r#"
                DELETE FROM auths WHERE user_id = $1 RETURNING *;
            "#,
            user_id
        ).fetch_one(conn).await;

        let auth_sql = match res_auth_sql {
            Ok(auth_sql) => auth_sql,
            Err(err) => match err {
                sqlx::Error::RowNotFound => return Err("User not found".to_string()),
                _ => return Err(err.to_string()),
            }
        };

        let indentifications = get_identifications(conn, &user_id).await;
        let tokens_metadata = get_tokens_metadata(conn, &user_id).await;

        auth_sql.to_auth_domain(indentifications, tokens_metadata)
            .map_err(|err| err.to_string())
    }
}

async fn get_identifications(conn: &Pool<Postgres>, user_id: &Uuid) -> Result<Vec<IdentificationSQL>, sqlx::Error> { 
    let identifications = sqlx::query!(
        r#"
            SELECT * FROM identifications i WHERE i.user_id = $1;
        "#,
        user_id
    ).fetch_all(conn).await;
    match identifications {
        Ok(identifications) => Ok(identifications.iter()
            .map(|r| IdentificationSQL::from_pgrow(r)?).collect()),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err("User not found".to_string()),
            _ => Err(err.to_string()),
        }
    }
}

async fn get_tokens_metadata(conn: &Pool<Postgres>, user_id: &Uuid) -> Result<Vec<TokenMetadataSQL>, sqlx::Error> { 
    let tokens_metadata = sqlx::query!(
        r#"
            SELECT * FROM tokens_metadata tm WHERE tm.user_id = $1;
        "#,
        user_id
    ).fetch_all(conn).await;
    match tokens_metadata {
        Ok(tokens_metadata) => Ok(tokens_metadata.iter()
            .map(|r| TokenMetadataSQL::from_pgrow(r)?).collect()),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err("User not found".to_string()),
            _ => Err(err.to_string()),
        }
    }
}