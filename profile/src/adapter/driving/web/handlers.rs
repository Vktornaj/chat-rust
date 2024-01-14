use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};

use super::schemas::UserJson;
use crate::application::use_cases;
use common::adapter::state::AppState;

// Adapters
use crate::adapter::driven::persistence::sqlx::user_repository::UserRepository;


pub async fn handle_get_user_info(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
) -> Result<Json<UserJson>, StatusCode> {
    match use_cases::get_user_info::execute(
        &state.db_sql_pool,
        &UserRepository {},
        &state.config.secret,
        &token.token().to_string(),
    )
    .await
    {
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(err) => match err {
            use_cases::get_user_info::FindError::Unknown(_) => Err(StatusCode::NOT_FOUND),
            use_cases::get_user_info::FindError::Unautorized(_) => Err(StatusCode::UNAUTHORIZED),
        },
    }
}

pub async fn handle_update_user_info(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(user_info): Json<UserJson>,
) -> Result<Json<UserJson>, StatusCode> {
    match use_cases::update_user_info::execute(
        &state.db_sql_pool, 
        &UserRepository {}, 
        &state.config.secret, 
        &token.token().to_string(), 
        use_cases::update_user_info::Payload {
            first_name: user_info.first_name,
            last_name: user_info.last_name,
            birthday: user_info.birthday,
            nationality: user_info.nationality,
            languages: user_info.languages,
        }
    ).await {
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(err) => {
            match err {
                use_cases::update_user_info::UpdateError::Unknown(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                use_cases::update_user_info::UpdateError::Unautorized => Err(StatusCode::UNAUTHORIZED),
                _ => Err(StatusCode::BAD_REQUEST),
            }
        },
    }
}