use chrono::{DateTime, Utc};
use common::domain::types::{error::ErrorMsg, id::Id};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::domain::{
    user::User as UserDomain, types::{
        first_name::FirstName, 
        last_name::LastName, 
        birthday::Birthday, 
        nationality::Nationality, 
        language::Language, 
    }
};


pub struct User {
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub birthday: DateTime<Utc>,
    pub nationality: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl User {
    pub fn from_pgrow(row: &PgRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            user_id: row.try_get("user_id")?,
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            birthday: row.try_get("birthday")?,
            nationality: row.try_get("nationality")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }

    pub fn to_user_domain(self, languages:Vec<String>) -> Result<UserDomain, ErrorMsg> {
        let languages: Result<Vec<Language>, ErrorMsg> = languages.into_iter()
            .map(|x| Language::try_from(x)).collect();
        Ok(UserDomain {
            id: Id::try_from(self.user_id)?,
            first_name: FirstName::try_from(self.first_name)?,
            last_name: LastName::try_from(self.last_name)?,
            birthday: Birthday::try_from(self.birthday)?,
            nationality: Nationality::try_from(self.nationality)?,
            languages: languages?,
            created_at: self.created_at,
            updated_at: self.updated_at
        })
    }
}