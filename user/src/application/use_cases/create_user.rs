use chrono::{DateTime, Utc};

use super::{super::port::driven::user_repository::UserRepositoryTrait, utils};
use crate::{
    domain::user::{
        User, Email, PhoneNumber, Password, FirstName, LastName, Birthday, Nationality, Language
    }, 
    application::port::driven::user_repository::NewUser
};
use super::is_user_exist;


#[derive(Debug)]
pub enum CreateError {
    InvalidData(String),
    Unknown(String),
    Conflict(String)
}

pub async fn execute<T>(conn: &T, repo: &impl UserRepositoryTrait<T>, mut new_user: NewUser) -> Result<User, CreateError> {
    // validate data
    new_user = validate_data(new_user)?;  
    // verify no user with same email or phone number
    if is_user_exist::execute(conn, repo, &new_user.email, &new_user.phone_number).await {
        return Err(CreateError::Conflict("email or phone already in use".to_string()))
    }
    // hash password
    new_user.hashed_password = if let Ok(hashed_password) = utils::hash_password(new_user.password.unwrap()) {
        Some(hashed_password)
    } else {
        return Err(CreateError::InvalidData("Invalid password".to_string()));
    };
    new_user.password = None;
    // create user
    match repo.create(conn, new_user).await {
        Ok(user) => Ok(user),
        Err(error) => Err(CreateError::Unknown(format!("Unknown error: {:?}", error))),
    }
}

fn validate_data(mut new_user: NewUser) -> Result<NewUser, CreateError> {
    new_user.email = evaluate::<Email, String>(new_user.email)?;
    new_user.phone_number = evaluate::<PhoneNumber, String>(new_user.phone_number)?;
    new_user.password = evaluate::<Password, String>(new_user.password)?;
    new_user.first_name = evaluate::<FirstName, String>(Some(new_user.first_name))?.unwrap();
    new_user.last_name = evaluate::<LastName, String>(Some(new_user.last_name))?.unwrap();
    new_user.birthday = evaluate::<Birthday, DateTime<Utc>>(Some(new_user.birthday))?.unwrap();
    new_user.nationality = evaluate::<Nationality, String>(Some(new_user.nationality))?.unwrap();
    let mut temp_languages: Vec<String> = Vec::new();
    for language in new_user.languages {
        if let Ok(language) = evaluate::<Language, String>(Some(language)) {
            temp_languages.push(language.unwrap());
        } else {
            return Err(CreateError::InvalidData("Invalid language".to_string()));
        }
    }
    new_user.languages = temp_languages;   
    Ok(new_user)
}

fn evaluate<T,E>(item: Option<E>) -> Result<Option<E>, CreateError> 
where 
    T: std::convert::TryFrom<E>,
    E: std::convert::From<T>,
    <T as TryFrom<E>>::Error: std::fmt::Display
{

    if let Some(some) = item {
        match T::try_from(some) {
            Ok(some) => Ok(Some(some.into())),
            Err(some) => Err(CreateError::InvalidData(some.to_string()))
        }
    } else {
        Ok(None)
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    // use crate::repositories::pokemon::InMemoryRepository;
    use chrono::{Utc, NaiveDate};
    use rocket::tokio;
    use uuid::Uuid;
    use super::*;
    use crate::domain::user::Id;
    use crate::adapter::driven::persistence::in_memory_repository::InMemoryRepository;
    

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
        let new_user = NewUser {
            email: Some("some_2@some.some".to_string()),
            phone_number: Some("+528331114146".to_string()),
            password: Some("Password123!".to_string()),
            hashed_password: None,
            first_name: "Victor Eduardo".to_string(),
            last_name: "Garcia Najera".to_string(),
            birthday: NaiveDate::from_ymd_opt(1990, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap(),
            nationality: "MEX".to_string(),
            languages: vec!["ES".to_string(), "EN".to_string()],
        };

        let res = execute(&conn, &repo, new_user).await;
        
        match res {
            Ok(user) => {
                assert!(Id::try_from(Into::<Uuid>::into(user.id.unwrap())).is_ok());
                assert_eq!(user.email, Some(Email::try_from("some_2@some.some".to_string()).unwrap()));
                assert_eq!(user.phone_number, Some(PhoneNumber::try_from("+528331114146".to_string()).unwrap()));
                assert_eq!(user.first_name, Some(FirstName::try_from("Victor Eduardo".to_string()).unwrap()));
                assert_eq!(user.last_name, Some(LastName::try_from("Garcia Najera".to_string()).unwrap()));
                assert_eq!(user.birthday, Some(Birthday::try_from(NaiveDate::from_ymd_opt(1990, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Utc)
                    .unwrap()).unwrap()));
                assert_eq!(user.nationality, Nationality::try_from("MEX".to_string()).unwrap());
                assert_eq!(user.languages, Some(vec![
                    Language::try_from("ES".to_string()).unwrap(), 
                    Language::try_from("EN".to_string()).unwrap()
                ]));
            }   
            _ => unreachable!(),
        };
    }
}