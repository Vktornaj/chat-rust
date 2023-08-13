use chrono::{DateTime, Utc};
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

use crate::domain::{
    user::User as UserDomain, types::{
        id::Id, email::Email, 
        phone_number::PhoneNumber, 
        first_name::FirstName, 
        last_name::LastName, 
        birthday::Birthday, 
        nationality::Nationality, 
        language::Language, error::ErrorMsg
    }
};


pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub hashed_password: String,
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
            id: row.try_get("id")?,
            email: row.try_get("email")?,
            phone_number: row.try_get("phone_number")?,
            hashed_password: row.try_get("hashed_password")?,
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
            id: Id::try_from(self.id)?,
            email: self.email.map(|x| Email::try_from(x)).transpose()?,
            phone_number: self.phone_number.map(|x| PhoneNumber::try_from(x)).transpose()?,
            hashed_password: self.hashed_password,
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