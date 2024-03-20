use std::sync::Mutex;
use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    application::port::driven::auth_repository::{AuthRepositoryTrait, self, UpdateIdentify}, 
    domain::{auth::{Auth, NewAuth}, types::identification::{IdentificationValue, NewIdentification}}
};


pub struct InMemoryRepository();

#[async_trait]
impl AuthRepositoryTrait<Mutex<Vec<Auth>>> for InMemoryRepository {
    async fn find_by_id(&self, conn: &Mutex<Vec<Auth>>, user_id: Uuid) -> Result<Auth, auth_repository::Error> {
        // TODO: Implement this method
        todo!()
    }

    async fn find_by_identification(
        &self, 
        conn: &Mutex<Vec<Auth>>, 
        identification: IdentificationValue,
    ) -> Result<Option<Auth>, auth_repository::Error> {
        // TODO: Implement this method
        todo!()
    }

    async fn create(
        &self, 
        conn: &Mutex<Vec<Auth>>, 
        auth: NewAuth, 
    ) -> Result<Auth, auth_repository::Error> {
        // TODO: Implement this method
        todo!()
    }

    async fn update_password(
        &self, 
        conn: &Mutex<Vec<Auth>>, 
        user_id: Uuid, 
        new_hashed_password: String,
    ) -> Result<Auth, auth_repository::Error> {
        // TODO: Implement this method
        todo!()
    }

    async fn update_identifications(
        &self, 
        conn: &Mutex<Vec<Auth>>, 
        identification_operation: UpdateIdentify<NewIdentification, Uuid>
    ) -> Result<Auth, auth_repository::Error> {
        // TODO: Implement this method
        todo!()
    }

    async fn delete(&self, conn: &Mutex<Vec<Auth>>, user_id: Uuid) -> Result<Auth, auth_repository::Error> {
        // TODO: Implement this method
        todo!()
    }
}