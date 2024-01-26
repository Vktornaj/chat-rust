use crate::{
    application::port::driven::auth_repository::{AuthRepositoryTrait, RepoSelectError},
    domain::types::identification::IdentificationValue,
};

pub struct Payload {
    pub identify_value: String,
    pub identify_type: String,
}

pub async fn execute<T>(
    conn: &T,
    repo: &impl AuthRepositoryTrait<T>,
    payload: Payload,
) -> Result<bool, String> {
    let identification =
        IdentificationValue::from_string(payload.identify_value, payload.identify_type)?;

    match repo.find_by_identification(conn, identification).await {
        Ok(_) => Ok(false),
        Err(err) => match err {
            RepoSelectError::NotFound(_) => Ok(true),
            RepoSelectError::Unknown(err) => Err(err),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::execute;
    use crate::domain::auth::NewAuth;
    use crate::domain::types::identification::IdentificationValue;
    use crate::{
        adapter::driven::persistence::sqlx::auth_repository::AuthRepository,
        application::port::driven::auth_repository::AuthRepositoryTrait,
    };
    use common::adapter::db;
    use common::domain::types::email::Email;

    #[tokio::test]
    async fn not_exists() {
        let db_sql_pool = db::create_pool().await;
        let res = execute(
            &db_sql_pool,
            &AuthRepository {},
            super::Payload {
                identify_value: "none@none.none".to_string(),
                identify_type: "email".to_string(),
            },
        );
        assert_eq!(res.await.unwrap(), true);
    }

    #[tokio::test]
    async fn exists() {
        let db_sql_pool = db::create_pool().await;
        let auth_repository = AuthRepository();

        // create auth
        let new_auth = NewAuth {
            identifications: vec![IdentificationValue::Email(
                Email::try_from("some@some.some".to_string()).unwrap(),
            )],
            hashed_password: "".to_string(),
        };
        let id = match auth_repository.create(&db_sql_pool, new_auth).await {
            Err(_) => panic!("Failed to create auth"),
            Ok(auth) => auth.user_id,
        };

        let res = execute(
            &db_sql_pool,
            &auth_repository,
            super::Payload {
                identify_value: "some@some.some".to_string(),
                identify_type: "email".to_string(),
            },
        )
        .await
        .unwrap();

        // delete auth
        if auth_repository
            .delete(&db_sql_pool, id.into())
            .await
            .is_err()
        {
            panic!("Failed to delete auth");
        }
        assert_eq!(res, false);
    }
}
