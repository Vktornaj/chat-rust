// extern crate rocket;

use axum::extract::{State, Path, TypedHeader};
use axum::headers::authorization::Bearer;
use axum::http::StatusCode;
use axum::Json;
use axum::headers::Authorization;
use chrono::{TimeZone, Utc};

use common::adapter::config::{DATE_FORMAT, Environment};
use super::schemas::{
    Credentials, Credentials2, Credentials3, IdTransaction, JsonToken, NewUserJson,
    UserContactInfo, UserContactInfo2, UserInfo, UserJson, ValidTransaction,
};
use crate::adapter::driven::email_service::aws_ses_email_service::AWSEmailService;
use crate::application::use_cases;
use common::adapter::state::AppState;

// Adapters
use crate::adapter::driven::{
    cache::redis::user_cache::UserCache,
    email_service::fake_email_service::FakeEmailService,
    persistence::sqlx::user_repository::UserRepository,
};


pub async fn handle_create_user_cache(
    State(state): State<AppState>,
    Json(payload): Json<NewUserJson>,
) -> Result<Json<IdTransaction>, (StatusCode, String)> {
    let date = if let Ok(date) = Utc.datetime_from_str(&payload.birthday, DATE_FORMAT) {
        date
    } else {
        return Err((StatusCode::BAD_REQUEST, "Invalid birthday format".into()));
    };
    
    match use_cases::create_user_cache::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &state.email_conn,
        &UserRepository {},
        &UserCache {},
        &AWSEmailService {},
        &state.config.environment,
        use_cases::create_user_cache::Payload {
            email: payload.email,
            phone_number: payload.phone_number,
            password: payload.password,
            first_name: payload.first_name,
            last_name: payload.last_name,
            birthday: date,
            nationality: payload.nationality,
            languages: payload.languages,
        },
    )
    .await
    {
        Ok(user) => Ok(Json(IdTransaction {
            id_transaction: user,
        })),
        Err(error) => match error {
            use_cases::create_user_cache::CreateError::InvalidData(err) => {
                Err((StatusCode::BAD_REQUEST, err))
            }
            use_cases::create_user_cache::CreateError::Unknown(err) => {
                Err((StatusCode::INTERNAL_SERVER_ERROR, err))
            }
            use_cases::create_user_cache::CreateError::Conflict(err) => {
                Err((StatusCode::CONFLICT, err))
            }
        },
    }
}

pub async fn handle_create_user_confirmation(
    State(state): State<AppState>,
    Json(payload): Json<ValidTransaction>,
) -> Result<Json<UserJson>, (StatusCode, String)>  {
    match use_cases::create_user_validate::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &UserRepository {},
        &UserCache {},
        use_cases::create_user_validate::Payload {
            transaction_id: payload.transaction_id,
            confirmation_code: payload.confirmation_code
        }
    ).await {
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(error) => match error {
            use_cases::create_user_validate::CreateError::InvalidData(err) => Err((StatusCode::BAD_REQUEST, err)),
            use_cases::create_user_validate::CreateError::Unknown(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
            use_cases::create_user_validate::CreateError::Conflict(err) => Err((StatusCode::CONFLICT, err)),
        }
    }
}

pub async fn handle_email_available(
    State(state): State<AppState>,
    Path(email): Path<String>,
) -> StatusCode {
    let res = use_cases::is_data_in_use::execute(
        &state.db_sql_pool,
        &UserRepository {},
        use_cases::is_data_in_use::Payload {
            email: Some(email),
            phone_number: None
        }
    ).await;
    match res {
        Ok(is_in_use) => {
            if is_in_use {
                StatusCode::CONFLICT
            } else {
                StatusCode::OK
            }
        },
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub async fn handle_phone_number_available(
    State(state): State<AppState>,
    Path(phone_number): Path<String>,
) -> StatusCode {
    let res = use_cases::is_data_in_use::execute(
        &state.db_sql_pool,
        &UserRepository {},
        use_cases::is_data_in_use::Payload {
            email: None,
            phone_number: Some(phone_number)
        }
    ).await;
    match res {
        Ok(is_in_use) => {
            if is_in_use {
                StatusCode::CONFLICT
            } else {
                StatusCode::OK
            }
        },
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

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

pub async fn handle_login(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<JsonToken>, StatusCode> {
    match use_cases::login_user::execute(
        &state.db_sql_pool,
        &UserRepository {},
        &state.config.secret,
        use_cases::login_user::Payload {
            email: credentials.email,
            phone_number: credentials.phone_number,
            password: credentials.password,
        },
    )
    .await
    {
        Ok(token) => Ok(Json(JsonToken {
            authorization_token: token,
            token_type: "Bearer".to_string(),
        })),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn handle_delete_account(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(credentials): Json<Credentials2>,
) -> StatusCode {
    match use_cases::delete_user::execute(
        &state.db_sql_pool,
        &UserRepository {},
        &state.config.secret,
        &token.token().to_string(),
        use_cases::delete_user::Payload {
            password: credentials.password,
        },
    )
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}

pub async fn handle_update_password(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(credentials): Json<Credentials3>,
) -> StatusCode {
    match use_cases::update_password::execute(
        &state.db_sql_pool,
        &UserRepository {},
        &state.config.secret,
        &token.token().to_string(),
        use_cases::update_password::Payload {
            password: credentials.password,
            new_password: credentials.new_password,
        },
    )
    .await
    {
        Ok(_) => StatusCode::OK,
        Err(err) => {
            match err {
                use_cases::update_password::UpdateError::Unknown(_) => StatusCode::INTERNAL_SERVER_ERROR,
                use_cases::update_password::UpdateError::Unautorized => StatusCode::UNAUTHORIZED,
                _ => StatusCode::BAD_REQUEST,
            }
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

pub async fn handle_update_user_contact_info_cache(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(user_info): Json<UserContactInfo>,
) -> Result<String, StatusCode> {
    match use_cases::update_contact_info_cache::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &state.email_conn,
        &UserRepository {},
        &UserCache {},
        &AWSEmailService {},
        &state.config.secret,
        &token.token().to_string(),
        use_cases::update_contact_info_cache::Payload {
            email: user_info.email,
            phone_number: user_info.phone_number,
        }
    ).await {
        Ok(res) => {
            match res {
                Some(user) => Ok(user),
                None => Err(StatusCode::NOT_FOUND),
            }
        },
        Err(err) => {
            match err {
                use_cases::update_contact_info_cache::UpdateError::Unknown(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                use_cases::update_contact_info_cache::UpdateError::Unautorized => Err(StatusCode::UNAUTHORIZED),
                _ => Err(StatusCode::BAD_REQUEST),
            }
        },
    }
}

pub async fn handle_update_user_contact_info_confirmation(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<ValidTransaction>,
) -> Result<Json<UserJson>, StatusCode> {
    match use_cases::update_contact_info_validate::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &UserRepository {},
        &UserCache {},
        &state.config.secret,
        &token.token().to_string(),
        use_cases::update_contact_info_validate::Payload {
            transaction_id: data.transaction_id,
            confirmation_code: data.confirmation_code
        }
    ).await {
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(err) => {
            match err {
                use_cases::update_contact_info_validate::UpdateError::Unknown(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                use_cases::update_contact_info_validate::UpdateError::Unautorized => Err(StatusCode::UNAUTHORIZED),
                _ => Err(StatusCode::BAD_REQUEST),
            }
        },
    }
}

// TODO: use different secret for password reset
// TODO: get domain from config
pub async fn handle_password_recovery_request(
    State(state): State<AppState>,
    Json(data): Json<UserContactInfo2>,
) -> Result<(), StatusCode> {
    match use_cases::reset_password_request::execute(
        &state.db_sql_pool,
        &state.email_conn,
        &UserRepository {},
        &AWSEmailService {},
        &state.config.secret,
        use_cases::reset_password_request::Payload {
            email: data.email,
            phone_number: data.phone_number,
            domain: "localhost:8000".to_string(),
        }
    ).await {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

// TODO: use different secret for password reset
// TODO: get domain from config
pub async fn handle_password_reset_confirmation(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Json(data): Json<Credentials2>,
) -> Result<Json<UserJson>, StatusCode> {
    match use_cases::reset_password::execute(
        &state.db_sql_pool,
        &UserRepository {},
        &state.config.secret,
        use_cases::reset_password::Payload {
            token,
            password: data.password,
        }
    ).await {
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}