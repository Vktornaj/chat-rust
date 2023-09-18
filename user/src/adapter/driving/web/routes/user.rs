extern crate rocket;

use deadpool::managed::Pool;
use deadpool_redis::{Manager, Connection};
use chrono::{Utc, TimeZone};
use common::config::DATE_FORMAT;
use lettre::SmtpTransport;
use rocket::http::{Status, ContentType};
use rocket::response::status;
use rocket::{get, post, State, delete, put};
use rocket::serde::json::Json;
use sqlx::PgPool;

use crate::adapter::driving::web::schemas::user::{
    NewUserJson, 
    UserJson, 
    UserInfo, 
    Credentials, 
    JsonToken, 
    Credentials2, 
    Credentials3, 
    UserContactInfo, 
    IdTransaction, ValidTransaction, UserContactInfo2
};
use crate::application::use_cases;
use common::{config::AppState, token::Token};

// Adapters
use crate::adapter::driven::persistence::sqlx::user_repository::UserRepository;
use crate::adapter::driven::cache::redis::user_cache::UserCache;
use crate::adapter::driven::email_service::fake_email_service::FakeEmailService;


#[post("/register", format = "json", data = "<user>")]
pub async fn create_user_cache(
    pool: &rocket::State<PgPool>, 
    cache_pool: &rocket::State<Pool<Manager, Connection>>,
    user: Json<NewUserJson>,
    state: &State<AppState>,
) -> Result<Json<IdTransaction>, (Status, String)>  {
    let date = if let Ok(date) = Utc.datetime_from_str(&user.birthday, DATE_FORMAT) {
        date
    } else {
        return Err((Status::BadRequest, "Invalid birthday format".into()));
    };
    let email_conn = if state.environment == "production" {
        Some(SmtpTransport::relay("smtp.gmail.com").unwrap().build())
    } else {
        None
    };
    match use_cases::create_user_cache::execute(
        pool.inner(),
        cache_pool.inner(),
        &email_conn,
        &UserRepository {},
        &UserCache {},
        &FakeEmailService {},
        &state.environment,
        use_cases::create_user_cache::Payload {
            email: user.0.email,
            phone_number: user.0.phone_number,
            password: user.0.password,
            first_name: user.0.first_name,
            last_name: user.0.last_name,
            birthday: date,
            nationality: user.0.nationality,
            languages: user.0.languages
        }
    ).await {
        Ok(user) => Ok(Json(IdTransaction { id_transaction: user })),
        Err(error) => match error {
            use_cases::create_user_cache::CreateError::InvalidData(err) => Err((Status::BadRequest, err)),
            use_cases::create_user_cache::CreateError::Unknown(err) => Err((Status::InternalServerError, err)),
            use_cases::create_user_cache::CreateError::Conflict(err) => Err((Status::Conflict, err)),
        }
    }
}

#[post("/register-confirmation", format = "json", data = "<data>")]
pub async fn create_user_confirmation(
    pool: &rocket::State<PgPool>, 
    cache_pool: &rocket::State<Pool<Manager, Connection>>, 
    data: Json<ValidTransaction>
) -> Result<Json<UserJson>, (Status, String)>  {
    match use_cases::create_user_validate::execute(
        pool.inner(),
        cache_pool.inner(),
        &UserRepository {},
        &UserCache {},
        use_cases::create_user_validate::Payload {
            transaction_id: data.0.transaction_id,
            confirmation_code: data.0.confirmation_code
        }
    ).await {
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(error) => match error {
            use_cases::create_user_validate::CreateError::InvalidData(err) => Err((Status::BadRequest, err)),
            use_cases::create_user_validate::CreateError::Unknown(err) => Err((Status::InternalServerError, err)),
            use_cases::create_user_validate::CreateError::Conflict(err) => Err((Status::Conflict, err)),
        }
    }
}

#[get("/email-availability/<email>")]
pub async fn email_available(
    pool: &rocket::State<PgPool>, 
    email: String
) -> (Status, (ContentType, String)) {
    let res = use_cases::is_data_in_use::execute(
        pool.inner(),
        &UserRepository {}, 
        use_cases::is_data_in_use::Payload {
            email: Some(email),
            phone_number: None
        }
    ).await;
    let is_available = if let Ok(res) = res {
        !res
    } else {
        return (Status::InternalServerError, (ContentType::Plain, "".into()));
    };
    
    (
        Status::Ok,
        (ContentType::JSON, format!("{{ \"isAvailable\": \"{is_available}\" }}"))
    )
}

#[get("/phone-availability/<phone_number>")]
pub async fn phone_number_available(
    pool: &rocket::State<PgPool>, 
    phone_number: String
) -> (Status, (ContentType, String)) {
    let res = use_cases::is_data_in_use::execute(
        pool.inner(),
        &UserRepository {}, 
        use_cases::is_data_in_use::Payload {
            email: None,
            phone_number: Some(phone_number)
        }
    ).await;
    let is_available = if let Ok(res) = res {
        !res
    } else {
        return (Status::InternalServerError, (ContentType::Plain, "".into()));
    };
    (
        Status::Ok,
        (ContentType::JSON, format!("{{ \"isAvailable\": \"{is_available}\" }}"))
    )
}

#[get("/user/info")]
pub async fn get_user_info(
    pool: &rocket::State<PgPool>,
    state: &State<AppState>, 
    token: Token
) -> Result<Json<UserJson>, Status> {
    match use_cases::get_user_info::execute(
        pool.inner(),
        &UserRepository {},
        &state.secret,
        &token.value
    ).await {
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(err) => match err {
            use_cases::get_user_info::FindError::Unknown(_) => Err(Status::NotFound),
            use_cases::get_user_info::FindError::Unautorized(_) => Err(Status::Unauthorized),
        },
    }   
}

#[post("/login", format = "json", data = "<credentials>")]
pub async fn login(
    pool: &rocket::State<PgPool>,
    state: &State<AppState>,
    credentials: Json<Credentials>,
) -> Result<Json<JsonToken>, status::Unauthorized<String>> {
    let invalid_msg = "invalid credentials".to_string();
    match use_cases::login_user::execute(
        pool.inner(),
        &UserRepository {},
        &state.secret,
        use_cases::login_user::Payload {
            email: credentials.0.email,
            phone_number: credentials.0.phone_number,
            password: credentials.0.password,
        }
    ).await {
        Ok(token) => Ok(Json(JsonToken { 
            authorization_token: token, 
            token_type: "Bearer".to_string() 
        })),
        Err(_) => Err(status::Unauthorized(Some(invalid_msg))),
    }
}

#[delete("/account", format = "json", data = "<credentials>")]
pub async fn delete_account(
    pool: &rocket::State<PgPool>,
    state: &State<AppState>,
    token: Token,
    credentials: Json<Credentials2>,
) -> Status {
    match use_cases::delete_user::execute(
        pool.inner(),
        &UserRepository {},
        &state.secret,
        &token.value,
        use_cases::delete_user::Payload {
            password: credentials.0.password,
        }
    ).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::Unauthorized,
    }
}

#[put("/password", format = "json", data = "<credentials>")]
pub async fn update_password(
    pool: &rocket::State<PgPool>,
    state: &State<AppState>,
    token: Token,
    credentials: Json<Credentials3>,
) -> Status {
    match use_cases::update_password::execute(
        pool.inner(), 
        &UserRepository {}, 
        &state.secret,
        &token.value,
        use_cases::update_password::Payload {
            password: credentials.0.password,
            new_password: credentials.0.new_password,
        }
    ).await {
        Ok(_) => Status::Ok,
        Err(_) => Status::Unauthorized
    }
}

#[put("/user-info", format = "json", data = "<user_info>")]
pub async fn update_user_info(
    pool: &rocket::State<PgPool>,
    state: &State<AppState>,
    token: Token,
    user_info: Json<UserInfo>,
) -> Result<Json<UserJson>, status::BadRequest<String>>  {
    let date = if let Some(date) = user_info.0.birthday {
        match Utc.datetime_from_str(&date, DATE_FORMAT) {
            Ok(date) => Some(date),
            Err(_) => return Err(status::BadRequest(Some("Invalid date format".into()))),
        }
    } else {
        None
    };
    match use_cases::update_user_info::execute(
        pool.inner(), 
        &UserRepository {}, 
        &state.secret,
        &token.value,
        use_cases::update_user_info::Payload {
            first_name: user_info.0.first_name,
            last_name: user_info.0.last_name,
            birthday: date,
            nationality: user_info.0.nationality,
            languages: user_info.0.languages,
        }
    ).await {
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(e) => Err(status::BadRequest(Some(format!("{:?}", e)))),
    }
}

#[put("/user-contact-info", format = "json", data = "<user_contact_info>")]
pub async fn update_user_contact_info_cache(
    pool: &rocket::State<PgPool>,
    cache_pool: &rocket::State<Pool<Manager, Connection>>,
    state: &State<AppState>,
    token: Token,
    user_contact_info: Json<UserContactInfo>,
) -> Result<Option<String>, status::BadRequest<String>> {
    let email_conn = if state.environment == "production" {
        Some(SmtpTransport::relay("smtp.gmail.com").unwrap().build())
    } else {
        None
    };
    match use_cases::update_contact_info_cache::execute(
        pool.inner(), 
        cache_pool.inner(),
        &email_conn,
        &UserRepository {},
        &UserCache {},
        &FakeEmailService {},
        &state.secret,
        &token.value,
        use_cases::update_contact_info_cache::Payload {
            email: user_contact_info.0.email,
            phone_number: user_contact_info.0.phone_number,
        }
    ).await {
        Ok(user) => Ok(user),
        Err(e) => Err(status::BadRequest(Some(format!("{:?}", e)))),
    }
}

#[put("/user-contact-info-confirmation", format = "json", data = "<data>")]
pub async fn update_user_contact_info_confirmation(
    pool: &rocket::State<PgPool>,
    cache_pool: &rocket::State<Pool<Manager, Connection>>,
    state: &State<AppState>,
    token: Token,
    data: Json<ValidTransaction>,
) -> Result<Json<UserJson>, (Status, String)>  {
    match use_cases::update_contact_info_validate::execute(
        pool.inner(),
        cache_pool.inner(),
        &UserRepository {},
        &UserCache {},
        &state.secret,
        &token.value,
        use_cases::update_contact_info_validate::Payload {
            transaction_id: data.0.transaction_id,
            confirmation_code: data.0.confirmation_code
        }
    ).await {
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(error) => match error {
            use_cases::update_contact_info_validate::UpdateError::InvalidData(err) => Err((Status::BadRequest, err)),
            use_cases::update_contact_info_validate::UpdateError::Unknown(err) => Err((Status::InternalServerError, err)),
            use_cases::update_contact_info_validate::UpdateError::Conflict(err) => Err((Status::Conflict, err)),
            use_cases::update_contact_info_validate::UpdateError::NotFound => Err((Status::NotFound, "User not found".into())),
            use_cases::update_contact_info_validate::UpdateError::Unautorized => Err((Status::Unauthorized, "Unautorized".into())),
        }
    }
}

// TODO: use different secret for password reset
// TODO: get domain from config
#[put("/password-reset-request", format = "json", data = "<data>")]
pub async fn password_reset_request(
    pool: &rocket::State<PgPool>,
    state: &State<AppState>,
    data: Json<UserContactInfo2>,
) -> Result<(), status::BadRequest<String>> {
    let email_conn = if state.environment == "production" {
        Some(SmtpTransport::relay("smtp.gmail.com").unwrap().build())
    } else {
        None
    };
    match use_cases::reset_password_request::execute(
        pool.inner(),
        &email_conn,
        &UserRepository {},
        &FakeEmailService {},
        &state.secret,
        use_cases::reset_password_request::Payload {
            email: data.0.email,
            phone_number: data.0.phone_number,
            domain: "localhost:8000".to_string(),
        }
    ).await {
        Ok(_) => Ok(()),
        Err(e) => Err(status::BadRequest(Some(format!("{:?}", e)))),
    }
}

// TODO: use different secret for password reset
// TODO: get domain from config
#[put("/password-reset/<token>", format = "json", data = "<data>")]
pub async fn password_reset(
    pool: &rocket::State<PgPool>,
    state: &State<AppState>,
    data: Json<Credentials2>,
    token: String,
) -> Result<Json<UserJson>, status::BadRequest<String>> {
    match use_cases::reset_password::execute(
        pool.inner(),
        &UserRepository {},
        &state.secret,
        use_cases::reset_password::Payload {
            token,
            password: data.0.password,
        }
    ).await {
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(e) => Err(status::BadRequest(Some(format!("{:?}", e)))),
    }
}