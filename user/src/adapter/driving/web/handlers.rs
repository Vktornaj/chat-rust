use axum::extract::{State, TypedHeader};
use axum::headers::authorization::Bearer;
use axum::http::StatusCode;
use axum::Json;
use axum::headers::Authorization;
use chrono::{TimeZone, Utc};

use common::adapter::config::DATE_FORMAT;
use super::schemas::{UserInfo, UserJson};
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
    Json(user_info): Json<UserInfo>,
) -> Result<Json<UserJson>, StatusCode> {
    // TODO: move this to application layer
    let date = if let Some(date) = user_info.birthday {
        match Utc.datetime_from_str(&date, DATE_FORMAT) {
            Ok(date) => Some(date),
            Err(_) => return Err(StatusCode::BAD_REQUEST),
        }
    } else {
        None
    };
    match use_cases::update_user_info::execute(
        &state.db_sql_pool, 
        &UserRepository {}, 
        &state.config.secret, 
        &token.token().to_string(), 
        use_cases::update_user_info::Payload {
            first_name: user_info.first_name,
            last_name: user_info.last_name,
            birthday: date,
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