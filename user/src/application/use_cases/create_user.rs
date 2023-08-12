use super::{super::port::driven::user_repository::UserRepositoryTrait, utils};
use crate::{
    domain::user::User, 
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
    // verify no user with same email or phone number
    if is_user_exist::execute(conn, repo, &new_user.email, &new_user.phone_number).await {
        return Err(CreateError::Conflict("email or phone already in use".to_string()))
    }
    // hash password
    new_user.hashed_password = if let Ok(hashed_password) = utils::hash_password(new_user.password.unwrap().into()) {
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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    // use crate::repositories::pokemon::InMemoryRepository;
    use chrono::{Utc, NaiveDate};
    use rocket::tokio;
    use uuid::Uuid;
    use super::*;
    use crate::domain::user::{Id, Email, PhoneNumber, Password, FirstName, LastName, Birthday, Nationality, Language};
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
            email: Some(Email::try_from("some_2@some.some".to_string()).unwrap()),
            phone_number: Some(PhoneNumber::try_from("+528331114146".to_string()).unwrap()),
            password: Some(Password::try_from("Password123!".to_string()).unwrap()),
            hashed_password: None,
            first_name: FirstName::try_from("Victor Eduardo".to_string()).unwrap(),
            last_name: LastName::try_from("Garcia Najera".to_string()).unwrap(),
            birthday: Birthday::try_from(
                NaiveDate::from_ymd_opt(1990, 1, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap()
                    .and_local_timezone(Utc)
                    .unwrap()
            ).unwrap(),
            nationality: Nationality::try_from("MEX".to_string()).unwrap(),
            languages: vec![
                Language::try_from("ES".to_string()).unwrap(), 
                Language::try_from("EN".to_string()).unwrap(), 
            ],
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