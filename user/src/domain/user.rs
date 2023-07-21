use chrono::{DateTime, Utc};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString, Error
    },
    Argon2
};

pub struct User {
    pub id: Option<i32>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: Option<DateTime<Utc>>,
    pub nationality: String,
    pub languages: Option<Vec<String>>,
    pub created_at:  Option<DateTime<Utc>>,
    pub updated_at:  Option<DateTime<Utc>>,
}

impl User {
    // TODO: Reduce the runtime; 1.3 seconds
    pub fn hash_password_mut(&mut self) -> Result<(), Error>{
        let salt = SaltString::generate(&mut OsRng);
        
        let argon2 = Argon2::default();
        self.password = argon2.hash_password(self.password.as_bytes(), &salt)?
            .to_string();
        Ok(())
    }

    // TODO: Reduce the runtime; 1.2 seconds
    pub fn verify_password(&self, password: &String) -> Result<(), Error> {
        let parsed_hash = PasswordHash::new(&self.password)?;
        Argon2::default().verify_password(
            password.as_bytes(), 
            &parsed_hash
        )
    }
}

pub struct NewUser {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birthday: DateTime<Utc>,
    pub nationality: Option<String>,
    pub languages: Option<Vec<String>>,
}