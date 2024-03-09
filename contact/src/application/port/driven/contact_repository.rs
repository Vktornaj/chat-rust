use async_trait::async_trait;

use common::domain::types::id::Id;
use crate::domain::contact::{Contact, NewContact};


pub enum Error {
    NotFound,
    DatabaseError,
}

pub struct UpdateContact {
    pub id: Id,
    pub user_id: Id,
    pub alias: Option<Option<String>>,
    pub blocked: Option<bool>,
}

#[async_trait]
pub trait ContactRepositoryTrait<T> {
    /// Get a contact by its id
    /// 
    /// # Parameters
    /// - `conn` - The connection to the database
    /// - `user_id` - The id of the user
    /// - `id` - The id of the contact
    /// 
    /// # Returns
    /// - `Ok(Contact)` - The contact
    /// - `Err(Error)` - An error occurred
    async fn get_by_id(&self, conn: &T, user_id: Id, id: Id) -> Result<Contact, Error>;

    /// Get all contacts of a user
    /// 
    /// # Parameters
    /// - `conn` - The connection to the database
    /// - `user_id` - The id of the user
    /// 
    /// # Returns
    /// - `Ok(Vec<Contact>)` - The contacts
    /// - `Err(Error)` - An error occurred
    async fn get_by_user_id(&self, conn: &T, user_id: Id) -> Result<Vec<Contact>, Error>;

    /// Create a new contact
    /// 
    /// # Parameters
    /// - `conn` - The connection to the database
    /// - `new_contact` - The new contact
    /// 
    /// # Returns
    /// - `Ok(Contact)` - The created contact
    /// - `Err(Error)` - An error occurred
    async fn create(&self, conn: &T, new_contact: NewContact) -> Result<Contact, Error>;

    /// Update a contact
    /// 
    /// # Parameters
    /// - `conn` - The connection to the database
    /// - `update_contact` - The update contact
    /// 
    /// # Returns
    /// - `Ok(Contact)` - The updated contact
    /// - `Err(Error)` - An error occurred
    async fn update(&self, conn: &T, update_contact: UpdateContact) -> Result<Contact, Error>;

    /// Delete a contact
    /// 
    /// # Parameters
    /// - `conn` - The connection to the database
    /// - `user_id` - The id of the user
    /// - `id` - The id of the contact
    ///
    /// # Returns
    /// - `Ok(())` - The contact was deleted
    /// - `Err(Error)` - An error occurred
    async fn delete(&self, conn: &T, user_id: Id, id: Id) -> Result<(), Error>;
}
