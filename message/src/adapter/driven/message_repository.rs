use axum::async_trait;
use uuid::Uuid;
use mongodb::{bson::{Document, doc}, options::FindOptions, Client, Collection};
use futures::TryStreamExt;

use common::domain::types::{recipient::Recipient, sender_type::Sender};
use crate::{
    application::port::driven::message_repository::{Error, MessageRepositoryTrait, UpdateMessage}, 
    domain::message::{Message, NewMessage}
};


pub struct MessageRepository();

#[async_trait]
impl MessageRepositoryTrait<Client> for MessageRepository {
    async fn create(&self, conn: &Client, new_message: NewMessage) -> Result<Message, Error> {
        let message = Message {
            id: Uuid::new_v4(),
            sender: new_message.sender,
            recipient: new_message.recipient,
            message_type: new_message.message_type,
            content: new_message.content,
            deleted: false,
            received_at: None,
            read_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        let collection: Collection<Message> = conn.database("chat_app").collection("messages");
        let res = collection.insert_one(&message, None).await;
        match res {
            Ok(_) => Ok(message),
            Err(err) => Err(Error::DatabaseError(err.to_string())),
        }
    }
    
    async fn find_list(
        &self, 
        conn: &Client, 
        sender: Sender,
        recipient: Recipient,
        limit: i64, 
        offset: Option<u64>, 
        ascending: bool,
    ) -> Result<Vec<Message>, Error> {
        let collection: Collection<Message> = conn.database("chat_app").collection("messages");
        let filter = doc! {
            "sender": Into::<String>::into(sender),
            "recipient": Into::<String>::into(recipient),
        };
        let options = FindOptions::builder()
            .limit(limit)
            .skip(offset)
            .sort(doc! {
                "created_at": if ascending { 1 } else { -1 },
            })
            .build();
        let mut cursor = collection.find(filter, options).await
            .map_err(|err| Error::DatabaseError(err.to_string()))?;
        let mut messages = Vec::new();
        while let Some(message) = cursor.try_next().await
            .map_err(|err| Error::DatabaseError(err.to_string()))?
        {
            messages.push(message);
        }
        Ok(messages)
    }
    
    async fn find_by_id(&self, conn: &Client, id: Uuid) -> Result<Message, Error> {
        let collection: Collection<Message> = conn.database("chat_app").collection("messages");
        let result = collection.find_one(doc! { "id": Into::<String>::into(id) }, None).await
            .map_err(|err| Error::DatabaseError(err.to_string()))?;
        match result {
            Some(message) => Ok(message),
            None => Err(Error::NotFound("".to_string())),
        }
    }
    
    async fn update(&self, conn: &Client, message: &UpdateMessage) -> Result<Message, Error> {
        let collection: Collection<Message> = conn.database("chat_app").collection("messages");
        let filter = doc! { "id": Into::<String>::into(message.id) };

        let mut doc = Document::new();

        if let Some(received_at) = message.received_at {
            doc.insert("received_at", received_at.map(|x| x.to_string()));
        }

        if let Some(read_at) = message.read_at {
            doc.insert("read_at", read_at.map(|x| x.to_string()));
        }
        
        let update = doc! { "$set": doc };

        let result = collection.update_one(filter, update, None).await
            .map_err(|err| Error::DatabaseError(err.to_string()))?;
        match result.modified_count {
            1 => Ok(self.find_by_id(conn, message.id).await?),
            _ => Err(Error::NotFound("".to_string())),
        }
    }
    
    async fn delete(&self, conn: &Client, id: Uuid) -> Result<(), Error> {
        let collection: Collection<Message> = conn.database("chat_app").collection("messages");
        let result = collection.delete_one(doc! { "id": Into::<String>::into(id) }, None).await
            .map_err(|err| Error::DatabaseError(err.to_string()))?;
        match result.deleted_count {
            1 => Ok(()),
            _ => Err(Error::NotFound("".to_string())),
        }
    }
}
