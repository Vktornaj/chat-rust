use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::user::{
    User as UserDomain, 
    Id, 
    Email, 
    PhoneNumber, 
    Password, 
    FirstName, 
    LastName, 
    Birthday, 
    Nationality, 
    Language
};


pub struct User {
    pub id: Uuid,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub hashed_password: String,
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
            id: Some(Id::try_from(self.id).unwrap()),
            email: self.email.map(|x| Email::try_from(x).unwrap()),
            phone_number: self.phone_number.map(|x| PhoneNumber::try_from(x).unwrap()),
            password: None,
            hashed_password: Some(self.hashed_password),
            first_name: self.first_name.map(|x| FirstName::try_from(x).unwrap()),
            last_name: self.last_name.map(|x| LastName::try_from(x).unwrap()),
            birthday: Some(Birthday::try_from(self.birthday).unwrap()),
            nationality: Nationality::try_from(self.nationality).unwrap(),
            languages: languages.map(|x| x.into_iter()
                .map(|x| Language::try_from(x).unwrap())
                .collect::<Vec<Language>>()),
            created_at: Some(self.created_at),
            updated_at: Some(self.updated_at)
        }
    }
}