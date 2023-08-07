use std::sync::Mutex;
use async_trait::async_trait;
use chrono::{Utc, DateTime};
use uuid::Uuid;

use super::{user_repository::{UserRepositoryTrait, NewUser, UpdateUser, FindUser}, errors};
use crate::domain::user::{User as UserDomain, Email, PhoneNumber, Id, Password, FirstName, LastName, Birthday, Nationality, Language};

pub struct InMemoryRepository();

#[async_trait]
impl UserRepositoryTrait<Mutex<Vec<UserDomain>>> for InMemoryRepository {
    async fn find_by_id(&self, conn: &Mutex<Vec<UserDomain>>, id: Uuid) -> Result<UserDomain, errors::RepoSelectError> {
        todo!()
    }

    async fn find_by_criteria(
        &self, 
        conn: &Mutex<Vec<UserDomain>>,
        find_user: &FindUser,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<UserDomain>, errors::RepoSelectError> {
        let lock = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(errors::RepoSelectError::Unknown("Failed to lock mutex".to_string()))
        };
        let res = lock.iter()
        .filter(|x| {
            if let Some(email) = &find_user.email {
                if x.email != Some(Email::try_from(email.clone()).unwrap()) {
                    return false
                }
            }
            if let Some(phone_number) = &find_user.phone_number {
                if x.phone_number != Some(PhoneNumber::try_from(phone_number.clone()).unwrap()) {
                    return false
                }
            }
            if let Some(birthday) = &find_user.birthday {
                if let Some(birthday_from) = &birthday.0 {
                    if &Into::<DateTime<Utc>>::into(x.birthday.clone().unwrap()) < birthday_from {
                        return false
                    }
                }
                if let Some(birthday_to) = &birthday.1 {
                    if &Into::<DateTime<Utc>>::into(x.birthday.clone().unwrap()) > birthday_to {
                        return false
                    }
                }
            }
            if let Some(nationality) = &find_user.nationality {
                if &Into::<String>::into(x.nationality.clone()) == nationality {
                    return false
                }
            }
            if let Some(languages) = &find_user.languages {
                if !x.languages.clone().unwrap().iter()
                    .any(|l| languages.contains(&Into::<String>::into(l.clone()))) {
                    return false
                }
            }
            return true
        })
        .skip(offset as usize)
        .take(limit as usize)
        .map(|x| x.clone())
        .collect();
        Ok(res)
    }

    async fn create(&self, conn: &Mutex<Vec<UserDomain>>, mut new_user: NewUser) -> Result<UserDomain, errors::RepoCreateError> {
        let mut lock: std::sync::MutexGuard<'_, Vec<UserDomain>> = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(errors::RepoCreateError::Unknown("Failed to lock mutex".to_string()))
        };
        if let Some(email) = new_user.email {
            if lock.iter().any(|u| u.email == Some(Email::try_from(email.clone()).unwrap())) {
                return Err(errors::RepoCreateError::Conflict("Email already in use".to_string()))
            }
            new_user.email = Some(email);
        }
        if let Some(phone_number) = new_user.phone_number {
            if lock.iter().any(|u| u.phone_number == Some(PhoneNumber::try_from(phone_number.clone()).unwrap())) {
                return Err(errors::RepoCreateError::Conflict("Phone number already in use".to_string()))
            }
            new_user.phone_number = Some(phone_number);
        }
        let user = UserDomain {
            id: Some(Id::try_from(uuid::Uuid::new_v4()).unwrap()),
            email: Some(Email::try_from(new_user.email.unwrap()).unwrap()),
            phone_number: Some(PhoneNumber::try_from(new_user.phone_number.unwrap()).unwrap()),
            // TODO: find a solution for hashed password
            password: None,
            hashed_password: Some(new_user.password),
            first_name: Some(FirstName::try_from(new_user.first_name).unwrap()),
            last_name: Some(LastName::try_from(new_user.last_name).unwrap()),
            birthday: Some(Birthday::try_from(new_user.birthday).unwrap()),
            nationality: Nationality::try_from(new_user.nationality).unwrap(),
            languages: Some(new_user.languages.into_iter().map(|x| (Language::try_from(x).unwrap())).collect()),
            created_at: Some(Utc::now()),
            updated_at: Some(Utc::now())
        };
        lock.push(user.clone());
        Ok(user)
    }

    async fn update(&self, conn: &Mutex<Vec<UserDomain>>, user: UpdateUser) -> Result<UserDomain, errors::RepoUpdateError> {
        todo!()
    }

    async fn delete(&self, conn: &Mutex<Vec<UserDomain>>, id: Uuid) -> Result<UserDomain, errors::RepoDeleteError> {
        todo!()
    }
}