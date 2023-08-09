use argon2::{
    password_hash::{
        PasswordHash, PasswordVerifier, Error
    },
    Argon2
};

// TODO: Reduce the runtime; 1.2 seconds
pub fn verify_password(user_password: &String, password: &String) -> Result<(), Error> {
    let parsed_hash = PasswordHash::new(&user_password)?;
    Argon2::default().verify_password(
        password.as_bytes(), 
        &parsed_hash
    )
}
