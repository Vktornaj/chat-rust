use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::user::User as UserDomain;


pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: DateTime<Utc>,
    pub nationality: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

impl User {
    pub fn to_user_domain(self, languages:Option<Vec<String>>) -> UserDomain {
        UserDomain {
            id: Some(self.id),
            email: self.email,
            phone_number: self.phone_number,
            password: self.password,
            first_name: self.first_name,
            last_name: self.last_name,
            birthday: Some(self.birthday),
            nationality: self.nationality,
            languages,
            created_at: Some(self.created_at),
            updated_at: Some(self.updated_at)
        }
    }
}