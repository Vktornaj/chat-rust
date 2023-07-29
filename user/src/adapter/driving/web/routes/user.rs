extern crate rocket;

use rocket::http::{Status, ContentType};
use rocket::response::status;
use rocket::{get, post, State};
use rocket::serde::{json::Json, Serialize, Deserialize};
use sqlx::PgPool;

use crate::adapter::driving::web::schemas::user::{NewUserJson, UserJson};
use crate::application::use_cases;
use common::{config::AppState, token::Token};

// Persistence
use crate::adapter::driven::persistence::sqlx::user_repository::UserRepository;


#[post("/register", format = "json", data = "<user>")]
pub async fn create_user(pool: &rocket::State<PgPool>, user: Json<NewUserJson>) -> Result<Json<UserJson>, Status>  {
    match use_cases::create_user::execute(
        pool.inner(),
        &UserRepository {},
        user.to_user()
    ).await {
        Ok(user) => Ok(Json(UserJson::from_user(user))),
        Err(error) => match error {
            use_cases::create_user::CreateError::InvalidData(_) => Err(Status::BadRequest),
            use_cases::create_user::CreateError::Unknown(_) => Err(Status::InternalServerError),
            use_cases::create_user::CreateError::Conflict(_) => Err(Status::Conflict),
        }
    }
}

#[get("/email-availability/<email>/<phone_number>")]
pub async fn username_available(
    pool: &rocket::State<PgPool>, 
    email: String, 
    phone_number: String
) -> (Status, (ContentType, String)) {
    let is_available = !use_cases::is_user_exist::execute(
        pool.inner(),
        &UserRepository {}, 
        &Some(email),
        &Some(phone_number)
    ).await;
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

#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    phone_number: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonToken {
    pub authorization_token: String,
    pub token_type: String,
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
        &Some(credentials.0.email), 
        &Some(credentials.0.phone_number), 
        &credentials.0.password
    ).await {
        Ok(token) => Ok(Json(JsonToken { 
            authorization_token: token, 
            token_type: "Bearer".to_string() 
        })),
        Err(_) => Err(status::Unauthorized(Some(invalid_msg))),
    }
}