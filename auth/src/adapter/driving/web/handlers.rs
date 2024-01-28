use axum::extract::{State, Path, Query};
use axum::http::StatusCode;
use axum::Json;
use axum_extra::TypedHeader;
use axum_extra::headers::Authorization;
use axum_extra::headers::authorization::Bearer;
use common::adapter::response_schemas::JsonResponse;

use crate::adapter::driven::cache::redis::user_cache::AuthCache;
use crate::adapter::driven::email_service::aws_ses_email_service::AWSEmailService;
use crate::adapter::driven::persistence::sqlx::auth_repository::AuthRepository;
use crate::application::use_cases::{create_auth_request, create_auth_confirm, is_data_in_use, login_auth, delete_auth, update_password, add_identy_request, add_identy_confirm, reset_password_request, reset_password_confirm};
use crate::schemas::{UuidWrapper, PasswordJson, JsonBool, ResOk};
use common::adapter::state::AppState;
use super::schemas::{AuthJson, ValidateTransaction, Credentials, JsonToken, UpdatePassword, IdentificationJson};


pub async fn handle_create_auth_request(
    State(state): State<AppState>,
    Json(payload): Json<AuthJson>,
) -> JsonResponse<IdentificationJson> {
    match create_auth_request::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &state.email_conn,
        &AuthRepository {},
        &AuthCache {},
        &AWSEmailService {},
        &state.config.environment,
        create_auth_request::Payload {
            password: payload.password,
            identification_type: payload.identifications[0].id_type.clone(),
            identification_value: payload.identifications[0].value.clone(),
        },
    ).await {
        Ok(identification) => JsonResponse::new_ok(IdentificationJson { 
            id_type: identification.get_type(), 
            value: identification.get_value() 
        }),
        Err(error) => match error {
            create_auth_request::CreateError::InvalidData(err) => JsonResponse::new_bad_req_err(0, err),
            create_auth_request::CreateError::Conflict(err) => JsonResponse::new_conflict_err(0, err),
            create_auth_request::CreateError::Unknown(err) => JsonResponse::new_int_ser_err(0, err),
        },
    }
}

pub async fn handle_create_auth_confirmation(
    State(state): State<AppState>,
    Json(payload): Json<ValidateTransaction>,
) -> JsonResponse<JsonToken> {
    match create_auth_confirm::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &AuthRepository {},
        &AuthCache {},
        &state.config.secret,
        create_auth_confirm::Payload {
            transaction_id: payload.transaction_id,
            confirmation_code: payload.confirmation_code
        }
    ).await {
        Ok(auth) => JsonResponse::new_ok(JsonToken {
            authorization_token: auth,
            token_type: "Bearer".to_string(),
        }),
        Err(error) => match error {
            create_auth_confirm::CreateError::InvalidData(err) => JsonResponse::new_bad_req_err(0, err),
            create_auth_confirm::CreateError::Conflict(err) => JsonResponse::new_conflict_err(0, err),
            create_auth_confirm::CreateError::Unknown(err) => JsonResponse::new_int_ser_err(0, err),
        }
    }
}

// TODO: improve error handling
pub async fn handle_identifier_available(
    State(state): State<AppState>,
    Query(identifier): Query<IdentificationJson>,
) -> JsonResponse<JsonBool> {
    let res = is_data_in_use::execute(
        &state.db_sql_pool,
        &AuthRepository {},
        is_data_in_use::Payload {
            identify_type: identifier.id_type,
            identify_value: identifier.value,
        }
    ).await;
    match res {
        Ok(is_in_use) => JsonResponse::new_ok(JsonBool { value: !is_in_use }),
        Err(err) => JsonResponse::new_int_ser_err(0, err)
    }
}

pub async fn handle_login(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> JsonResponse<JsonToken> {
    match login_auth::execute(
        &state.db_sql_pool,
        &AuthRepository {},
        &state.config.secret,
        login_auth::Payload {
            identifier: credentials.identifier,
            password: credentials.password,
        },
    )
    .await
    {
        Ok(token) => JsonResponse::new_ok(JsonToken {
            authorization_token: token,
            token_type: "Bearer".to_string(),
        }),
        Err(err) => match err {
            _ => JsonResponse::new_unauthorized_err(0, "invalid credentials".to_string()),
        },
    }
}

pub async fn handle_delete_account(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(password): Json<PasswordJson>,
) -> JsonResponse<ResOk> {
    match delete_auth::execute(
        &state.db_sql_pool,
        &AuthRepository {},
        &state.config.secret,
        &token.token().to_string(),
        delete_auth::Payload { password: password.password },
    ).await {
        Ok(_) => JsonResponse::new_ok(ResOk { ok: true }),
        Err(err) => {
            JsonResponse::new_err(StatusCode::UNAUTHORIZED, 0, "", err.to_string())
        },
    }
}

pub async fn handle_update_password(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(update_password): Json<UpdatePassword>,
) -> JsonResponse<ResOk> {
    match update_password::execute(
        &state.db_sql_pool,
        &AuthRepository {},
        &state.config.secret,
        &token.token().to_string(),
        update_password::Payload {
            password: update_password.password,
            new_password: update_password.new_password,
        },
    ).await {
        Ok(_) => JsonResponse::new_ok(ResOk { ok: true }),
        Err(err) => {
            match err {
                update_password::UpdateError::InvalidData(err) => JsonResponse::new_bad_req_err(0, err),
                update_password::UpdateError::Unauthorized(err) => JsonResponse::new_unauthorized_err(0, err),
                update_password::UpdateError::NotFound(err) => JsonResponse::new_not_found_err(0, err),
                update_password::UpdateError::Conflict(err) => JsonResponse::new_conflict_err(0, err),
                update_password::UpdateError::Unknown(err) => JsonResponse::new_int_ser_err(0, err),
            }
        },
    }
}

pub async fn handle_add_identifier_request(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(identifier): Json<IdentificationJson>,
) -> JsonResponse<ResOk> {
    match add_identy_request::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &state.email_conn,
        &AuthRepository {},
        &AuthCache {},
        &AWSEmailService {},
        &state.config.secret,
        &token.token().to_string(),
        add_identy_request::Payload {
            identify_value: identifier.value,
            identify_type: identifier.id_type,
        }
    ).await {
        Ok(_) => JsonResponse::new_ok(ResOk { ok: true }),
        Err(err) => {
            match err {
                add_identy_request::UpdateError::Unauthorized(err) => JsonResponse::new_unauthorized_err(0, err),
                add_identy_request::UpdateError::NotFound(err) => JsonResponse::new_not_found_err( 0, err),
                add_identy_request::UpdateError::InvalidData(err) => JsonResponse::new_bad_req_err(0, err),
                add_identy_request::UpdateError::Conflict(err) => JsonResponse::new_conflict_err(0, err),
                add_identy_request::UpdateError::Unknown(err) => JsonResponse::new_int_ser_err(0, err),
            }
        },
    }
}

pub async fn handle_add_identifier_confirmation(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(data): Json<ValidateTransaction>,
) -> JsonResponse<ResOk> {
    match add_identy_confirm::execute(
        &state.db_sql_pool,
        &state.cache_pool,
        &AuthRepository {},
        &AuthCache {},
        &state.config.secret,
        &token.token().to_string(),
        add_identy_confirm::Payload {
            transaction_id: data.transaction_id,
            confirmation_code: data.confirmation_code
        }
    ).await {
        Ok(_) => JsonResponse::new_ok( ResOk { ok: true } ),
        Err(err) => {
            match err {
                add_identy_confirm::UpdateError::InvalidData(err) => JsonResponse::new_bad_req_err(0, err),
                add_identy_confirm::UpdateError::Unauthorized(err) => JsonResponse::new_unauthorized_err(0, err),
                add_identy_confirm::UpdateError::Unknown(err) => JsonResponse::new_int_ser_err(0, err),
            }
        },
    }
}

// TODO: use different secret for password reset
// TODO: get domain from config
pub async fn handle_password_recovery_request(
    State(state): State<AppState>,
    Json(identifier): Json<IdentificationJson>,
) -> JsonResponse<ResOk> {
    match reset_password_request::execute(
        &state.db_sql_pool,
        &state.email_conn,
        &AuthRepository {},
        &AWSEmailService {},
        &state.config.secret,
        reset_password_request::Payload {
            identifier_type: identifier.id_type,
            identifier_value: identifier.value,
            domain: "localhost:8000".to_string(),
        }
    ).await {
        Ok(_) => JsonResponse::new_ok(ResOk { ok: true }),
        Err(err) => match err {
            reset_password_request::ResetError::InvalidData(err) => JsonResponse::new_bad_req_err(0, err),
            reset_password_request::ResetError::NotFound(err) => JsonResponse::new_not_found_err(0, err),
            reset_password_request::ResetError::Unknown(err) => JsonResponse::new_int_ser_err(0, err),
        },
    }
}

// TODO: use different secret for password reset
// TODO: get domain from config
pub async fn handle_password_reset_confirmation(
    State(state): State<AppState>,
    Path(token): Path<String>,
    Json(new_password): Json<PasswordJson>,
) -> JsonResponse<UuidWrapper> {
    match reset_password_confirm::execute(
        &state.db_sql_pool,
        &AuthRepository {},
        &state.config.secret,
        reset_password_confirm::Payload {
            token,
            password: new_password.password,
        }
    ).await {
        Ok(auth) => JsonResponse::new_ok(UuidWrapper { uuid: auth.user_id.into() }),
        Err(err) => match err {
            reset_password_confirm::ResetError::InvalidData(err) => JsonResponse::new_bad_req_err(0, err),
            reset_password_confirm::ResetError::Unauthorized(err) => JsonResponse::new_unauthorized_err(0, err),
            reset_password_confirm::ResetError::NotFound(err) => JsonResponse::new_not_found_err(0, err),
            reset_password_confirm::ResetError::Unknown(err) => JsonResponse::new_int_ser_err(0, err),
        }
    }
}
