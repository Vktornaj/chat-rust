// use argon2::{
//     password_hash::{
//         PasswordHash, PasswordVerifier, Error, SaltString, rand_core::OsRng
//     },
//     Argon2, PasswordHasher
// };

// // TODO: Reduce the runtime; 1.2 seconds
// pub fn verify_password(user_password: &String, password: &String) -> Result<(), Error> {
//     let parsed_hash = PasswordHash::new(&user_password)?;
//     Argon2::default().verify_password(
//         password.as_bytes(), 
//         &parsed_hash
//     )
// }

// // TODO: Reduce the runtime; 1.3 seconds
// pub fn hash_password(password: String) -> Result<String, Error>{
//     let salt = SaltString::generate(&mut OsRng);
    
//     let argon2 = Argon2::default();
//     Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
// }

// fn validate_data(mut new_user: NewUser) -> Result<NewUser, CreateError> {
//     new_user.email = evaluate::<Email, String>(new_user.email)?;
//     new_user.phone_number = evaluate::<PhoneNumber, String>(new_user.phone_number)?;
//     new_user.password = evaluate::<Password, String>(new_user.password)?;
//     new_user.first_name = evaluate::<FirstName, String>(Some(new_user.first_name))?.unwrap();
//     new_user.last_name = evaluate::<LastName, String>(Some(new_user.last_name))?.unwrap();
//     new_user.birthday = evaluate::<Birthday, DateTime<Utc>>(Some(new_user.birthday))?.unwrap();
//     new_user.nationality = evaluate::<Nationality, String>(Some(new_user.nationality))?.unwrap();
//     let mut temp_languages: Vec<String> = Vec::new();
//     for language in new_user.languages {
//         if let Ok(language) = evaluate::<Language, String>(Some(language)) {
//             temp_languages.push(language.unwrap());
//         } else {
//             return Err(CreateError::InvalidData("Invalid language".to_string()));
//         }
//     }
//     new_user.languages = temp_languages;   
//     Ok(new_user)
// }

// fn evaluate<T,E>(item: Option<E>) -> Result<Option<E>, CreateError> 
// where 
//     T: std::convert::TryFrom<E>,
//     E: std::convert::From<T>,
//     <T as TryFrom<E>>::Error: std::fmt::Display
// {

//     if let Some(some) = item {
//         match T::try_from(some) {
//             Ok(some) => Ok(Some(some.into())),
//             Err(some) => Err(CreateError::InvalidData(some.to_string()))
//         }
//     } else {
//         Ok(None)
//     }
// }