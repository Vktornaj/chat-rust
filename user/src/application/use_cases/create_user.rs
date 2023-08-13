use super::super::port::driven::user_repository::UserRepositoryTrait;
use crate::domain::user::{User, NewUser};
use super::is_data_in_use;


#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

pub struct Payload {
    pub email: Option<String>, 
    pub phone_number: Option<String>, 
    pub password: String, 
    pub first_name: String, 
    pub last_name: String, 
    pub birthday: chrono::DateTime<chrono::Utc>, 
    pub nationality: String, 
    pub languages: Vec<String>,
}

pub async fn execute<T>(conn: &T, repo: &impl UserRepositoryTrait<T>, payload: Payload) -> Result<User, CreateError> { 
    let new_user = match NewUser::new(
        payload.email,
        payload.phone_number, 
        payload.password, 
        payload.first_name, 
        payload.last_name, 
        payload.birthday, 
        payload.nationality, 
        payload.languages
    ) {
        Ok(new_user) => new_user,
        Err(error) => return Err(CreateError::InvalidData(error.to_string())),
    };
    // verify no user with same email or phone number
    let payload_idiu = is_data_in_use::Payload {
        email: new_user.email.as_ref().map(|x| x.to_owned().into()), 
        phone_number: new_user.phone_number.as_ref().map(|x| x.to_owned().into())
    };
    if let Ok(res) = is_data_in_use::execute(conn, repo, payload_idiu).await {
        if res {
            return Err(CreateError::Conflict("email or phone already in use".to_string()));
        }
    } else {
        return Err(CreateError::Unknown("unknown error".to_string()));
    }
    // create user
    match repo.create(conn, new_user).await {
        Ok(user) => Ok(user),
        Err(error) => Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    // use crate::repositories::pokemon::InMemoryRepository;
    use chrono::{Utc, NaiveDate, DateTime};
    use rocket::tokio;
    use uuid::Uuid;
    use super::*;
    use crate::{
        adapter::driven::persistence::in_memory_repository::InMemoryRepository, 
        domain::types::id::Id
    };
    

    // #[test]
    // fn it_should_return_a_bad_request_error_when_request_is_invalid() {
    //     let repo = Arc::new(InMemoryRepository::new());
    //     let req = Request::new(
    //         PokemonNumber::pikachu(),
    //         PokemonName::bad(),
    //         PokemonTypes::pikachu(),
    //     );

    //     let res = execute(repo, req);

    //     match res {
    //         Err(Error::BadRequest) => {}
    //         _ => unreachable!(),
    //     };
    // }

    // #[test]
    // fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
    //     let repo = Arc::new(InMemoryRepository::new());
    //     repo.insert(
    //         PokemonNumber::pikachu(),
    //         PokemonName::pikachu(),
    //         PokemonTypes::pikachu(),
    //     )
    //     .ok();
    //     let req = Request::new(
    //         PokemonNumber::pikachu(),
    //         PokemonName::charmander(),
    //         PokemonTypes::charmander(),
    //     );

    //     let res = execute(repo, req);

    //     match res {
    //         Err(Error::Conflict) => {}
    //         _ => unreachable!(),
    //     }
    // }

    // #[test]
    // fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
    //     let repo = Arc::new(InMemoryRepository::new().with_error());
    //     let req = Request::new(
    //         PokemonNumber::pikachu(),
    //         PokemonName::pikachu(),
    //         PokemonTypes::pikachu(),
    //     );

    //     let res = execute(repo, req);

    //     match res {
    //         Err(Error::Unknown) => {}
    //         _ => unreachable!(),
    //     };
    // }

    #[tokio::test]
    async fn it_should_return_the_user_otherwise() {
        let conn = Mutex::new(vec![]);
        let repo = InMemoryRepository {};
        let payload = Payload {
            email: Some("some_2@some.some".to_string()),
            phone_number: Some("+528331114146".to_string()),
            password: "Password123!".to_string(),
            first_name: "Victor".to_string(),
            last_name: "Najera".to_string(),
            birthday: NaiveDate::from_ymd_opt(1990, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap(),
            nationality: "MEX".to_string(),
            languages: vec!["ES".to_string(), "EN".to_string()]
        };

        let res = execute(&conn, &repo, payload).await;
        
        match res {
            Ok(user) => {
                assert!(Id::try_from(Into::<Uuid>::into(user.id)).is_ok());
                assert_eq!(Into::<String>::into(user.email.unwrap()), "some_2@some.some".to_string());
                assert_eq!(Into::<String>::into(user.phone_number.unwrap()), "+528331114146".to_string());
                assert_eq!(Into::<String>::into(user.first_name), "Victor".to_string());
                assert_eq!(Into::<String>::into(user.last_name), "Najera".to_string());
                assert_eq!(Into::<DateTime<Utc>>::into(user.birthday), NaiveDate::from_ymd_opt(1990, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Utc)
                    .unwrap());
                assert_eq!(Into::<String>::into(user.nationality), "MEX".to_string());
                assert_eq!(
                    user.languages.into_iter().map(|x| Into::<String>::into(x)).collect::<Vec<String>>(), 
                    vec!["ES".to_string(), "EN".to_string()]
                );
            }   
            _ => unreachable!(),
        };
    }
}