use axum::extract::State;
use axum::Json;
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};

use super::schemas::UserJson;
use crate::application::use_cases::{get_user_info, update_profile_info};
use common::adapter::{state::AppState, response_schemas::JsonResponse};

// Adapters
use crate::adapter::driven::persistence::sqlx::user_repository::UserRepository;


pub async fn handle_get_user_info(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
) -> JsonResponse<UserJson> {
    match get_user_info::execute(
        &state.db_sql_pool,
        &UserRepository {},
        &state.config.secret,
        &token.token().to_string(),
    )
    .await {
        Ok(user) => JsonResponse::new_ok(UserJson::from_user(user)),
        Err(err) => match err {
            get_user_info::FindError::Unknown(err) => JsonResponse::new_int_ser_err(0, err),
            get_user_info::FindError::Unauthorized(err) => JsonResponse::new_unauthorized_err(0, err)
        },
    }
}

pub async fn handle_update_user_info(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(user_info): Json<UserJson>,
) -> JsonResponse<UserJson> {
    match update_profile_info::execute(
        &state.db_sql_pool, 
        &UserRepository {}, 
        &state.config.secret, 
        &token.token().to_string(), 
        update_profile_info::Payload {
            first_name: user_info.firstname,
            last_name: user_info.lastname,
            birthday: user_info.birthday,
            nationality: user_info.nationality,
            languages: user_info.languages,
        }
    ).await {
        Ok(user) => JsonResponse::new_ok(UserJson::from_user(user)),
        Err(err) => {
            match err {
                update_profile_info::UpdateError::Unauthorized => JsonResponse::new_unauthorized_err(
                    1, 
                    err.to_string()
                ),
                _ => JsonResponse::new_int_ser_err(0, err.to_string()),
            }
        },
    }
}
