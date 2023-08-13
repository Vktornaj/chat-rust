extern crate rocket;

use chrono::{Utc, TimeZone};
use common::config::DATE_FORMAT;
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
    Credentials3
};
use crate::application::use_cases;
use common::{config::AppState, token::Token};

// Persistence
use crate::adapter::driven::persistence::sqlx::user_repository::UserRepository;


#[post("/register", format = "json", data = "<user>")]
pub async fn create_user(pool: &rocket::State<PgPool>, user: Json<NewUserJson>) -> Result<Json<UserJson>, (Status, String)>  {
    let date = if let Ok(date) = Utc.datetime_from_str(&user.birthday, DATE_FORMAT) {
        date
    } else {
        return Err((Status::BadRequest, "Invalid birthday format".into()));
    };
    match use_cases::create_user::execute(
        pool.inner(),
        &UserRepository {},
        use_cases::create_user::Payload {
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
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(error) => match error {
            use_cases::create_user::CreateError::InvalidData(err) => Err((Status::BadRequest, err)),
            use_cases::create_user::CreateError::Unknown(err) => Err((Status::InternalServerError, err)),
            use_cases::create_user::CreateError::Conflict(err) => Err((Status::Conflict, err)),
        }
    }
}

#[get("/email-availability/<email>")]
pub async fn email_available(
    pool: &rocket::State<PgPool>, 
    email: String, 
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

#[put("/user", format = "json", data = "<user_info>")]
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
