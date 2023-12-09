use std::sync::Mutex;
use async_trait::async_trait;
use chrono::Utc;
use common::domain::types::id::Id;
use uuid::Uuid;

// use super::{user_repository::{UserRepositoryTrait, NewUser, UpdateUser, FindUser}, errors};
use crate::{
    domain::{
        user::{User as UserDomain, NewUser}, 
        types::birthday::Birthday
    }, 
    application::port::driven::{
        errors, user_repository::{
            UserRepositoryTrait, FindUser, UpdateUser
        }
    }
};


pub struct InMemoryRepository();

#[async_trait]
impl UserRepositoryTrait<Mutex<Vec<UserDomain>>> for InMemoryRepository {
    async fn find_by_id(&self, conn: &Mutex<Vec<UserDomain>>, id: Uuid) -> Result<UserDomain, errors::RepoSelectError> {
        let lock = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(errors::RepoSelectError::Unknown("Failed to lock mutex".to_string()))
        };
        let id_ = if let Ok(id) = Id::try_from(id) {
            id
        } else {
            return Err(errors::RepoSelectError::Unknown("Failed to convert id".to_string()))
        };
        let res = lock.iter().filter(|x| x.id == id_)
            .map(|x| x).collect::<Vec<&UserDomain>>();
        if res.len() == 0 {
            return Err(errors::RepoSelectError::NotFound)
        }
        Ok(res[0].clone())
    }

    async fn find_by_criteria(
        &self, 
        conn: &Mutex<Vec<UserDomain>>,
        find_user: FindUser,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<UserDomain>, errors::RepoSelectError> {
        let lock = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(errors::RepoSelectError::Unknown("Failed to lock mutex".to_string()))
        };
        let res = lock.iter()
            .filter(|x| {
                if let Some(birthday) = &find_user.birthday {
                    if let Some(birthday_from) = birthday.0 {
                        if x.birthday < Birthday::try_from(birthday_from).unwrap() {
                            return false
                        }
                    }
                    if let Some(birthday_to) = birthday.1 {
                        if x.birthday > Birthday::try_from(birthday_to).unwrap() {
                            return false
                        }
                    }
                }
                if let Some(nationality) = &find_user.nationality {
                    if &x.nationality == nationality {
                        return false
                    }
                }
                if let Some(languages) = &find_user.languages {
                    if !x.languages.iter()
                        .any(|l| languages.contains(&l)) {
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
        let user = UserDomain {
            id: Id::try_from(uuid::Uuid::new_v4()).unwrap(),
            first_name: new_user.first_name,
            last_name: new_user.last_name,
            birthday: new_user.birthday,
            nationality: new_user.nationality,
            languages: new_user.languages,
            created_at: Utc::now(),
            updated_at: Utc::now()
        };
        lock.push(user.clone());
        Ok(user)
    }

    async fn update(&self, conn: &Mutex<Vec<UserDomain>>, update_user: UpdateUser) -> Result<UserDomain, errors::RepoUpdateError> {
        let mut lock: std::sync::MutexGuard<'_, Vec<UserDomain>> = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(errors::RepoUpdateError::Unknown("Failed to lock mutex".to_string()))
        };
        let id = if let Ok(id) = Id::try_from(update_user.id) {
            id
        } else {
            return Err(errors::RepoUpdateError::Unknown("Failed to convert id".to_string()))
        };

        let mut user = lock.iter_mut().find(|x| x.id == id).unwrap();

        if let Some(first_name) = update_user.first_name {
            user.first_name = first_name;
        }
        if let Some(last_name) = update_user.last_name {
            user.last_name = last_name;
        }
        if let Some(birthday) = update_user.birthday {
            user.birthday = birthday;
        }
        if let Some(nationality) = update_user.nationality {
            user.nationality = nationality;
        }
        Ok(user.clone())
    }

    async fn delete(&self, conn: &Mutex<Vec<UserDomain>>, id: Uuid) -> Result<UserDomain, errors::RepoDeleteError> {
        let mut lock: std::sync::MutexGuard<'_, Vec<UserDomain>> = match conn.lock() {
            Ok(lock) => lock,
            Err(_) => return Err(errors::RepoDeleteError::Unknown("Failed to lock mutex".to_string()))
        };
        let id = if let Ok(id) = Id::try_from(id) {
            id
        } else {
            return Err(errors::RepoDeleteError::Unknown("Failed to convert id".to_string()))
        };
        let index = lock.iter().position(|x| x.id == id).unwrap();
        Ok(lock.remove(index))  
    }
}