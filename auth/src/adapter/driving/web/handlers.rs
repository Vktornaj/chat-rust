use axum::extract::{State, Path, Query};
use axum::http::StatusCode;
use axum::Json;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use uuid::Uuid;

use crate::adapter::driven::cache::redis::user_cache::AuthCache;
use crate::adapter::driven::email_service::aws_ses_email_service::AWSEmailService;
use crate::adapter::driven::persistence::sqlx::user_repository::AuthRepository;
use crate::application::use_cases;
use crate::schemas::{UuidWrapper, PasswordJson};
use common::adapter::state::AppState;
use super::schemas::{AuthJson, ValidateTransaction, Credentials, JsonToken, UpdatePassword, IdentificationJson};


#[utoipa::path(
    post,
    path = "/api/auth/create-auth-request",
    request_body = AuthJson,
    responses(
        (status = 200, description = "create auth request", body = String),
        (status = 400, description = "invalid data"),
        (status = 409, description = "conflict"),
        (status = 500, description = "unknown error"),
    )
)]
pub async fn handle_create_auth_request(
    State(state): State<AppState>,
    Json(payload): Json<AuthJson>,
) -> Result<Json<String>, (StatusCode, String)> {
    match use_cases::create_auth_request::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &state.email_conn,
        &AuthRepository {},
        &AuthCache {},
        &AWSEmailService {},
        &state.config.environment,
        use_cases::create_auth_request::Payload {
            password: payload.password,
            identification_type: payload.identifications[0].id_type.clone(),
            identification_value: payload.identifications[0].value.clone(),
        },
    ).await {
        Ok(user) => Ok(Json(user)),
        Err(error) => match error {
            use_cases::create_auth_request::CreateError::InvalidData(err) => {
                Err((StatusCode::BAD_REQUEST, err))
            }
            use_cases::create_auth_request::CreateError::Unknown(err) => {
                Err((StatusCode::INTERNAL_SERVER_ERROR, err))
            }
            use_cases::create_auth_request::CreateError::Conflict(err) => {
                Err((StatusCode::CONFLICT, err))
            }
        },
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/create-auth-confirmation",
    request_body = ValidateTransaction,
    responses(
        (status = 200, description = "create auth confirmation", body = UuidWrapper),
        (status = 400, description = "invalid data"),
        (status = 409, description = "conflict"),
        (status = 500, description = "unknown error")
    )
)]
pub async fn handle_create_auth_confirmation(
    State(state): State<AppState>,
    Json(payload): Json<ValidateTransaction>,
) -> Result<Json<UuidWrapper>, (StatusCode, String)>  {
    match use_cases::create_auth_confirm::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &AuthRepository {},
        &AuthCache {},
        use_cases::create_auth_confirm::Payload {
            transaction_id: payload.transaction_id,
            confirmation_code: payload.confirmation_code
        }
    ).await {
        Ok(auth) => Ok(Json(UuidWrapper{ uuid: auth.user_id.into() })),
        Err(error) => match error {
            use_cases::create_auth_confirm::CreateError::InvalidData(err) => Err((StatusCode::BAD_REQUEST, err)),
            use_cases::create_auth_confirm::CreateError::Conflict(err) => Err((StatusCode::CONFLICT, err)),
            use_cases::create_auth_confirm::CreateError::Unknown(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
        }
    }
}

// TODO: improve error handling
#[utoipa::path(
    get,
    path = "/api/auth/identifier-available",
    responses(
        (status = 200, description = "available"),
        (status = 409, description = "unavailable"),
    ),
    params (
        ("value" = String, Query, description = "identifier value"),
        ("idType" = String, Query, description = "identifier type"),
    )
)]
pub async fn handle_identifier_available(
    State(state): State<AppState>,
    Query(identifier): Query<IdentificationJson>,
) -> StatusCode {
    let res = use_cases::is_data_in_use::execute(
        &state.db_sql_pool,
        &AuthRepository {},
        use_cases::is_data_in_use::Payload {
            identify_type: identifier.id_type,
            identify_value: identifier.value,
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
        Err(_) => {
            // StatusCode::BAD_REQUEST
            StatusCode::OK
        },
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/login",
    request_body = Credentials,
    responses(
        (status = 200, description = "login", body = JsonToken),
        (status = 401, description = "unauthorized"),
    )
)]
pub async fn handle_login(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<JsonToken>, StatusCode> {
    match use_cases::login_auth::execute(
        &state.db_sql_pool,
        &AuthRepository {},
        &state.config.secret,
        use_cases::login_auth::Payload {
            identifier: credentials.identifier,
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

#[utoipa::path(
    delete,
    path = "/api/auth/auth",
    responses(
        (status = 200, description = "account deleted"),
        (status = 401, description = "unauthorized"),
    ),
    request_body = PasswordJson,
)]
pub async fn handle_delete_account(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(password): Json<PasswordJson>,
) -> StatusCode {
    match use_cases::delete_auth::execute(
        &state.db_sql_pool,
        &AuthRepository {},
        &state.config.secret,
        &token.token().to_string(),
        use_cases::delete_auth::Payload { password: password.password },
    ).await {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::UNAUTHORIZED,
    }
}

#[utoipa::path(
    put,
    path = "/api/auth/password",
    request_body = UpdatePassword,
    responses(
        (status = 200, description = "update password"),
        (status = 400, description = "invalid data"),
        (status = 401, description = "unauthorized"),
        (status = 500, description = "unknown error"),
    )
)]
pub async fn handle_update_password(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(update_password): Json<UpdatePassword>,
) -> StatusCode {
    match use_cases::update_password::execute(
        &state.db_sql_pool,
        &AuthRepository {},
        &state.config.secret,
        &token.token().to_string(),
        use_cases::update_password::Payload {
            password: update_password.password,
            new_password: update_password.new_password,
        },
    ).await {
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

#[utoipa::path(
    post,
    path = "/api/auth/identifier-request",
    request_body = IdentificationJson,
    responses(
        (status = 200, description = "create auth confirmation", body = String),
        (status = 401, description = "unauthorized"),
        (status = 404, description = "not found"),
        (status = 500, description = "unknown error"),
    )
)]
pub async fn handle_add_identifier_request(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(identifier): Json<IdentificationJson>,
) -> Result<String, StatusCode> {
    match use_cases::add_identy_request::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &state.email_conn,
        &AuthRepository {},
        &AuthCache {},
        &AWSEmailService {},
        &state.config.secret,
        &token.token().to_string(),
        use_cases::add_identy_request::Payload {
            identify_value: identifier.value,
            identify_type: identifier.id_type,
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
                use_cases::add_identy_request::UpdateError::Unknown(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                use_cases::add_identy_request::UpdateError::Unautorized => Err(StatusCode::UNAUTHORIZED),
                _ => Err(StatusCode::BAD_REQUEST),
            }
        },
    }
}

#[utoipa::path(
    post,
    path = "/api/auth/identifier-confirmation",
    request_body = ValidateTransaction,
    responses(
        (status = 200, description = "create auth confirmation", body = UuidWrapper),
        (status = 400, description = "invalid data"),
        (status = 401, description = "unauthorized"),
        (status = 500, description = "unknown error"),
    )
)]
pub async fn handle_add_identifier_confirmation(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<ValidateTransaction>,
) -> Result<Json<UuidWrapper>, StatusCode> {
    match use_cases::add_identy_confirm::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &AuthRepository {},
        &AuthCache {},
        &state.config.secret,
        &token.token().to_string(),
        use_cases::add_identy_confirm::Payload {
            transaction_id: data.transaction_id,
            confirmation_code: data.confirmation_code
        }
    ).await {
        Ok(auth) => Ok(Json(UuidWrapper{ uuid: auth.user_id.into() })),
        Err(err) => {
            match err {
                use_cases::add_identy_confirm::UpdateError::Unknown(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
                use_cases::add_identy_confirm::UpdateError::Unautorized => Err(StatusCode::UNAUTHORIZED),
                _ => Err(StatusCode::BAD_REQUEST),
            }
        },
    }
}

// TODO: use different secret for password reset
// TODO: get domain from config
#[utoipa::path(
    post,
    path = "/api/auth/password-recovery-request",
    request_body = IdentificationJson,
    responses(
        (status = 200, description = "password recovery request"),
        (status = 400, description = "invalid data"),
    )
)]
pub async fn handle_password_recovery_request(
    State(state): State<AppState>,
    Json(identifier): Json<IdentificationJson>,
) -> Result<(), StatusCode> {
    match use_cases::reset_password_request::execute(
        &state.db_sql_pool,
        &state.email_conn,
        &AuthRepository {},
        &AWSEmailService {},
        &state.config.secret,
        use_cases::reset_password_request::Payload {
            identifier_type: identifier.id_type,
            identifier_value: identifier.value,
            domain: "localhost:8000".to_string(),
        }
    ).await {
        Ok(_) => Ok(()),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}

// TODO: use different secret for password reset
// TODO: get domain from config
#[utoipa::path(
    post,
    path = "/api/auth/password-recovery-confirmation/{token}",
    request_body = PasswordJson,
    responses(
        (status = 200, description = "password reset confirmation", body = UuidWrapper),
        (status = 400, description = "invalid data"),
    ),
    params(
        ("token" = String, Path, description = "recovery password token"),
    ),
)]
pub async fn handle_password_reset_confirmation(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Json(new_password): Json<PasswordJson>,
) -> Result<Json<Uuid>, StatusCode> {
    match use_cases::reset_password_confirm::execute(
        &state.db_sql_pool,
        &AuthRepository {},
        &state.config.secret,
        use_cases::reset_password_confirm::Payload {
            token,
            password: new_password.password,
        }
    ).await {
        Ok(auth) => Ok(Json(auth.user_id.into())),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}