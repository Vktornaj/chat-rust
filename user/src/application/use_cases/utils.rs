use argon2::{
    password_hash::{
        PasswordHash, PasswordVerifier, Error, SaltString, rand_core::OsRng
    },
    Argon2, PasswordHasher
};

// TODO: Reduce the runtime; 1.2 seconds
pub fn verify_password(user_password: &String, password: &String) -> Result<(), Error> {
    let parsed_hash = PasswordHash::new(&user_password)?;
    Argon2::default().verify_password(
        password.as_bytes(), 
        &parsed_hash
    )
}

// TODO: Reduce the runtime; 1.3 seconds
pub fn hash_password(password: String) -> Result<String, Error>{
    let salt = SaltString::generate(&mut OsRng);
    
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}