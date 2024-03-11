use axum::extract::{Query, State};
use axum::Json;
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use crate::schemas::NewContactJson;
use crate::{
    application::use_cases::{add_contact, get_contacts, remove_contact, update_contact},
    domain::contact::Contact,
};
use common::adapter::{response_schemas::JsonResponse, state::AppState};

// Adapters
use crate::adapter::driven::persistence::sqlx::contact_repository::ContactRepository;

use super::schemas::{IdJson, UpdateContactJson};

pub async fn handle_get_contacts(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
) -> JsonResponse<Vec<Contact>> {
    match get_contacts::execute(
        &state.db_sql_pool,
        &ContactRepository {},
        &state.config.secret,
        &token.token().to_string(),
    )
    .await
    {
        Ok(contacts) => JsonResponse::new_ok(contacts),
        Err(err) => match err {
            get_contacts::Error::Unauthorized => {
                JsonResponse::new_unauthorized_err(0, "".to_string())
            }
            get_contacts::Error::NotFound => JsonResponse::new_not_found_err(0, "".to_string()),
            _ => JsonResponse::new_int_ser_err(0, "".to_string()),
        },
    }
}

pub async fn handle_create_contact(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(contact_info): Json<NewContactJson>,
) -> JsonResponse<Contact> {
    match add_contact::execute(
        &state.db_sql_pool,
        &ContactRepository {},
        &state.config.secret,
        &token.token().to_string(),
        add_contact::Payload { 
            id: contact_info.id, 
            alias: contact_info.alias,
            is_blocked: contact_info.is_blocked,
        },
    )
    .await
    {
        Ok(contact) => JsonResponse::new_ok(contact),
        Err(err) => match err {
            add_contact::Error::Unauthorized => {
                JsonResponse::new_unauthorized_err(1, "".to_string())
            }
            add_contact::Error::NotFound => {
                JsonResponse::new_not_found_err(1, "".to_string())
            }
            _ => JsonResponse::new_int_ser_err(0, "".to_string()),
        },
    }
}

pub async fn handle_update_contact(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Json(contact_info): Json<UpdateContactJson>,
) -> JsonResponse<Contact> {
    match update_contact::execute(
        &state.db_sql_pool,
        &ContactRepository {},
        &state.config.secret,
        &token.token().to_string(),
        update_contact::Payload {
            id: contact_info.id,
            alias: contact_info.alias,
            is_blocked: contact_info.is_blocked,
        },
    )
    .await
    {
        Ok(contact) => JsonResponse::new_ok(contact),
        Err(err) => match err {
            update_contact::Error::Unauthorized => {
                JsonResponse::new_unauthorized_err(1, "".to_string())
            }
            update_contact::Error::NotFound => {
                JsonResponse::new_not_found_err(1, "".to_string())
            }
            _ => JsonResponse::new_int_ser_err(0, "".to_string()),
        },
    }
}

pub async fn handle_delete_contact(
    State(state): State<AppState>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    Query(params): Query<IdJson>,
) -> JsonResponse<String> {
    match remove_contact::execute(
        &state.db_sql_pool,
        &ContactRepository {},
        &state.config.secret,
        &token.token().to_string(),
        remove_contact::Payload { id: params.id },
    )
    .await
    {
        Ok(_) => JsonResponse::new_ok(String::from("Done")),
        Err(err) => match err {
            remove_contact::Error::Unauthorized => {
                JsonResponse::new_unauthorized_err(0, "".to_string())
            }
            remove_contact::Error::NotFound => {
                JsonResponse::new_not_found_err(0, "".to_string())
            }
            _ => JsonResponse::new_int_ser_err(0, "".to_string()),
        },
    }
}