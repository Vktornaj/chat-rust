use common::domain::types::id::Id;
use crate::domain::{contact::{Contact, NewContact}, types::alias::Alias};


pub enum Error {
    NotFound,
    DatabaseError,
}

pub struct UpdateContact {
    pub id: Id,
    pub user_id: Id,
    pub alias: Option<Option<Alias>>,
    pub is_blocked: Option<bool>,
}

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
    fn find_by_id(&self, conn: &T, user_id: Id, id: Id) -> impl std::future::Future<Output = Result<Contact, Error>> + Send;

    /// Get all contacts of a user
    /// 
    /// # Parameters
    /// - `conn` - The connection to the database
    /// - `user_id` - The id of the user
    /// 
    /// # Returns
    /// - `Ok(Vec<Contact>)` - The contacts
    /// - `Err(Error)` - An error occurred
    fn find_by_user_id(&self, conn: &T, user_id: Id) -> impl std::future::Future<Output = Result<Vec<Contact>, Error>> + Send;

    /// Create a new contact
    /// 
    /// # Parameters
    /// - `conn` - The connection to the database
    /// - `new_contact` - The new contact
    /// 
    /// # Returns
    /// - `Ok(Contact)` - The created contact
    /// - `Err(Error)` - An error occurred
    fn create(&self, conn: &T, new_contact: NewContact) -> impl std::future::Future<Output = Result<Contact, Error>> + Send;

    /// Update a contact
    /// 
    /// # Parameters
    /// - `conn` - The connection to the database
    /// - `update_contact` - The update contact
    /// 
    /// # Returns
    /// - `Ok(Contact)` - The updated contact
    /// - `Err(Error)` - An error occurred
    fn update(&self, conn: &T, update_contact: UpdateContact) -> impl std::future::Future<Output = Result<Contact, Error>> + Send;

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
    fn delete(&self, conn: &T, user_id: Id, id: Id) -> impl std::future::Future<Output = Result<(), Error>> + Send;
}
