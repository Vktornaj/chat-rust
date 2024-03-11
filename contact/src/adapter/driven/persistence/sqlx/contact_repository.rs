use sqlx::query_builder::QueryBuilder;
use sqlx::{Postgres, Pool};
use uuid::Uuid;

use common::domain::types::id::Id;
use contact_repository::ContactRepositoryTrait;
use crate::application::port::driven::contact_repository::{self, UpdateContact};
use crate::domain::contact::{Contact, NewContact};
use super::models::contact::ContactDB;


pub struct ContactRepository();

impl ContactRepositoryTrait<Pool<Postgres>> for ContactRepository {
    async fn find_by_id(&self, conn: &Pool<Postgres>, user_id: Id, id: Id) -> Result<Contact, contact_repository::Error> {
        let contact = sqlx::query_as!(
            ContactDB,
            r#"
                SELECT * FROM contacts WHERE user_id = $1 AND id = $2;
            "#,
            &Uuid::from(user_id),
            &Uuid::from(id)
        ).fetch_one(conn).await;

        match contact {
            Ok(contact) => Ok(contact.to_contact()),
            Err(e) => {
                match e {
                    sqlx::Error::RowNotFound => Err(contact_repository::Error::NotFound),
                    _ => Err(contact_repository::Error::DatabaseError)
                }
            }
        }
    }

    async fn find_by_user_id(&self, conn: &Pool<Postgres>, user_id: Id) -> Result<Vec<Contact>, contact_repository::Error> {
        let contacts = sqlx::query_as!(
            ContactDB,
            r#"
                SELECT * FROM contacts WHERE user_id = $1;
            "#,
            &Uuid::from(user_id)
        ).fetch_all(conn).await;

        match contacts {
            Ok(contacts) => Ok(contacts.into_iter().map(|c| c.to_contact()).collect()),
            Err(_) => Err(contact_repository::Error::DatabaseError)
        }
    }

    async fn create(&self, conn: &Pool<Postgres>, new_contact: NewContact) -> Result<Contact, contact_repository::Error> {
        let contact = sqlx::query_as!(
            ContactDB,
            r#"
                INSERT INTO contacts (id, user_id, alias, is_blocked)
                VALUES ($1, $2, $3, $4)
                RETURNING *;
            "#,
            &Uuid::from(new_contact.id),
            &Uuid::from(new_contact.user_id),
            new_contact.alias.map(|x| String::from(x)),
            new_contact.is_blocked
        ).fetch_one(conn).await;

        match contact {
            Ok(contact) => Ok(contact.to_contact()),
            Err(_) => Err(contact_repository::Error::DatabaseError)
        }
    }

    async fn update(&self, conn: &Pool<Postgres>, update_contact: UpdateContact) -> Result<Contact, contact_repository::Error> {
        let mut query_builder = QueryBuilder::new("UPDATE contacts SET ");
        let mut separated = query_builder.separated(", ");

        if let Some(alias) = update_contact.alias {
            separated.push("alias = ");
            separated.push_bind_unseparated(alias.map(|x| String::from(x)));
        }
        if let Some(alias) = update_contact.is_blocked {
            separated.push("is_blocked = ");
            separated.push_bind_unseparated(alias);
        }

        // Add the WHERE clause with the user ID
        separated.push_unseparated(" WHERE id =");
        separated.push_bind_unseparated(Uuid::from(update_contact.user_id));

        // Execute the update query
        match query_builder.build().execute(conn).await {
            Ok(result) => {
                if result.rows_affected() > 0 {
                    match self.find_by_id(conn, update_contact.user_id, update_contact.id).await {
                        Ok(updated_user) => return Ok(updated_user),
                        Err(_) => return Err(contact_repository::Error::DatabaseError),
                    }
                } else {
                    Err(contact_repository::Error::NotFound)
                }
            },
            Err(_) => return Err(contact_repository::Error::DatabaseError),
        }
    }

    async fn delete(&self, conn: &Pool<Postgres>, user_id: Id, id: Id) -> Result<(), contact_repository::Error> {
        let result = sqlx::query_as!(
            ContactDB,
            r#"
                DELETE FROM contacts WHERE user_id = $1 AND id = $2;
            "#,
            &Uuid::from(user_id),
            &Uuid::from(id)
        ).fetch_optional(conn).await;

        match result {
            Ok(result) => match result {
                Some(_) => Ok(()),
                None => Err(contact_repository::Error::NotFound)
            }
            Err(_) => Err(contact_repository::Error::DatabaseError)
        }
    }
}
