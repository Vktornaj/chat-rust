use std::fmt;
use chrono::{DateTime, Utc, Datelike};
use uuid::Uuid;
use regex::Regex;


#[derive(Debug)]
pub struct ErrorMsg(String);

impl fmt::Display for ErrorMsg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

#[derive(PartialEq, Clone)]
pub struct Id(Uuid);

impl TryFrom<Uuid> for Id {
    type Error = ErrorMsg;

    fn try_from(value: Uuid) -> Result<Self, Self::Error> {
        if value.is_nil() {
            return Err(ErrorMsg("Id is nil".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<Id> for Uuid {
    fn from(id: Id) -> Self {
        id.0
    }
}

#[cfg(test)]
mod tests_id {
    use super::*;

    #[test]
    fn test_id() {
        let id = Id::try_from(Uuid::new_v4());
        assert!(id.is_ok());
        let id = Id::try_from(Uuid::nil());
        assert!(id.is_err());
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Email(String);

impl TryFrom<String> for Email {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Email is empty".to_string()))
        }
        let re = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
        if !re.is_match(&value) {
            return Err(ErrorMsg("Email is invalid".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

#[cfg(test)]
mod tests_email {
    use super::*;

    #[test]
    fn test_email() {
        let email = Email::try_from("some@some.some".to_string());
        assert!(email.is_ok());
        let email = Email::try_from("some@some".to_string());
        assert!(email.is_err());
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PhoneNumber(String);

impl TryFrom<String> for PhoneNumber {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Phone number is empty".to_string()))
        }
        let re = Regex::new(r"^\+[0-9]{1,3}[0-9]{3,14}$").unwrap();
        if !re.is_match(&value) {
            return Err(ErrorMsg("Phone number is invalid".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<PhoneNumber> for String {
    fn from(phone_number: PhoneNumber) -> Self {
        phone_number.0
    }
}

#[cfg(test)]
mod tests_phone_number {
    use super::*;

    #[test]
    fn test_phone_number() {
        let phone_number = PhoneNumber::try_from("+380123456789".to_string());
        assert!(phone_number.is_ok());
        let phone_number = PhoneNumber::try_from("+380123456789012345678901234567890123456".to_string());
        assert!(phone_number.is_err());
        let phone_number = PhoneNumber::try_from("380123456789".to_string());
        assert!(phone_number.is_err());
        let phone_number = PhoneNumber::try_from("+380123a567890".to_string());
        assert!(phone_number.is_err());
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Password(String);

impl TryFrom<String> for Password {
    type Error = ErrorMsg;

    // TODO: test these regex
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Password is empty".to_string()))
        }
        if value.len() < 8 || value.len() > 64 {
            return Err(ErrorMsg("Password length should be between 8 and 64".to_string()))
        }
        // At least one uppercase
        if !Regex::new(r"[A-Z]").unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should contain at least one uppercase".to_string()))
        }
        // At least one lowercase
        if !Regex::new(r"[a-z]").unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should contain at least one lowercase".to_string()))
        }
        // At least one digit
        if !Regex::new(r"[0-9]").unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should contain at least one digit".to_string()))
        }
        // At least one special character
        if !Regex::new(r#"[!@#$%^&*(),.?\":{}|<>]"#).unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should contain at least one special character".to_string()))
        }
        // No whitespace
        if Regex::new(r"\s").unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should not contain whitespace".to_string()))
        }
        // No unicode
        if Regex::new(r"[^\x00-\x7F]").unwrap().is_match(&value) {
            return Err(ErrorMsg("Password should not contain unicode".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<Password> for String {
    fn from(password: Password) -> Self {
        password.0
    }
}

#[cfg(test)]
mod tests_password {
    use super::*;

    #[test]
    fn test_password() {
        // ok
        let password = Password::try_from("Passwo1!".to_string());
        assert!(password.is_ok());
        let password = Password::try_from("Password123%".to_string());
        assert!(password.is_ok());
        let password = Password::try_from("@PASSWORD1p,".to_string());
        assert!(password.is_ok());
        // error
        let password = Password::try_from("Password1".to_string());
        assert!(password.is_err());
        let password = Password::try_from("password1!".to_string());
        assert!(password.is_err());
        let password = Password::try_from("Password!".to_string());
        assert!(password.is_err());
        let password = Password::try_from("Pss1!".to_string());
        assert!(password.is_err());
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FirstName(String);

impl TryFrom<String> for FirstName {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("First name is empty".to_string()))
        }
        if value.len() > 50 {
            return Err(ErrorMsg("First name is too long, max length 50".to_string()))
        }
        // at least one letter
        if !Regex::new(r"[a-zA-Z]").unwrap().is_match(&value) {
            return Err(ErrorMsg("First name should contain at least one letter".to_string()))
        }
        // only letters and spaces
        if !Regex::new(r"^[a-zA-Z ]+$").unwrap().is_match(&value) {
            return Err(ErrorMsg("First name should contain only letters and spaces".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<FirstName> for String {
    fn from(first_name: FirstName) -> Self {
        first_name.0
    }
}

#[cfg(test)]
mod tests_first_name {
    use super::*;

    #[test]
    fn test_first_name() {
        let first_name = FirstName::try_from("Some".to_string());
        assert!(first_name.is_ok());
        let first_name = FirstName::try_from("Some Somes".to_string());
        assert!(first_name.is_ok());
        let first_name = FirstName::try_from("Some1".to_string());
        assert!(first_name.is_err());
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct LastName(String);

impl TryFrom<String> for LastName {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Last name is empty".to_string()))
        }
        if value.len() > 50 {
            return Err(ErrorMsg("Last name is too long, max length 50".to_string()))
        }
        // at least one letter
        if !Regex::new(r"[a-zA-Z]").unwrap().is_match(&value) {
            return Err(ErrorMsg("Last name should contain at least one letter".to_string()))
        }
        // only letters and spaces
        if !Regex::new(r"^[a-zA-Z ]+$").unwrap().is_match(&value) {
            return Err(ErrorMsg("Last name should contain only letters and spaces".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<LastName> for String {
    fn from(last_name: LastName) -> Self {
        last_name.0
    }
}

#[cfg(test)]
mod tests_last_name {
    use super::*;

    #[test]
    fn test_last_name() {
        let last_name = LastName::try_from("Some".to_string());
        assert!(last_name.is_ok());
        let last_name = LastName::try_from("Some Somes".to_string());
        assert!(last_name.is_ok());
        let last_name = LastName::try_from("Some1".to_string());
        assert!(last_name.is_err());
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd)]
pub struct Birthday(DateTime<Utc>);

impl TryFrom<DateTime<Utc>> for Birthday {
    type Error = ErrorMsg;

    fn try_from(value: DateTime<Utc>) -> Result<Self, Self::Error> {
        let now = Utc::now();
        if now.year() - value.year() < 13 {
            return Err(ErrorMsg("You must be at least 13 years old".to_string()))
        }
        if now.year() - value.year() == 13 && now.month() < value.month() {
            return Err(ErrorMsg("You must be at least 13 years old".to_string()))
        }
        if now.year() - value.year() == 13 
            && now.month() == value.month() 
            && now.day() < value.day() {
            return Err(ErrorMsg("You must be at least 13 years old".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<Birthday> for DateTime<Utc> {
    fn from(birthday: Birthday) -> Self {
        birthday.0
    }
}

#[cfg(test)]
mod tests_birthday {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn test_birthday() {
        let now = Utc::now();
        // ok
        let birthday = {
            let dt = NaiveDate::from_ymd_opt(now.year() - 14, 1, 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap();
            Birthday::try_from(dt)
        };
        assert!(birthday.is_ok());
        let birthday =  {
            let dt = NaiveDate::from_ymd_opt(now.year() - 13, now.month(), now.day())
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap();
            Birthday::try_from(dt)
        };
        assert!(birthday.is_ok());
        // error
        let birthday = {
            let dt = NaiveDate::from_ymd_opt(now.year() - 13, now.month(), now.day() + 1)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap();
            Birthday::try_from(dt)
        };
        assert!(birthday.is_err());
        let birthday = {
            let dt = NaiveDate::from_ymd_opt(now.year() - 13, now.month() + 1, now.day())
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_local_timezone(Utc)
                .unwrap();
            Birthday::try_from(dt)
        };
        assert!(birthday.is_err());
    }
}

// alpha-3 code (ISO 3166)
#[derive(PartialEq, Debug, Clone)]
pub struct Nationality(String);

impl TryFrom<String> for Nationality {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Nationality is empty".to_string()))
        }
        let re = Regex::new(r"^[A-Z]{3}$").unwrap();
        if !re.is_match(&value) {
            return Err(ErrorMsg("Nationality should be alpha-3 code (ISO 3166)".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<Nationality> for String {
    fn from(value: Nationality) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests_nationality {
    use super::*;

    #[test]
    fn test_nationality() {
        let nationality = Nationality::try_from("AAA".to_owned());
        assert!(nationality.is_ok());
        let nationality = Nationality::try_from("AA".to_owned());
        assert!(nationality.is_err());
        let nationality = Nationality::try_from("A".to_owned());
        assert!(nationality.is_err());
        let nationality = Nationality::try_from("A2".to_owned());
        assert!(nationality.is_err());
        let nationality = Nationality::try_from("AAA2".to_owned());
        assert!(nationality.is_err());
    }
}

// alpha-2 code (ISO 639-1)
#[derive(PartialEq, Debug, Clone)]
pub struct Language(String);

impl TryFrom<String> for Language {
    type Error = ErrorMsg;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(ErrorMsg("Language is empty".to_string()))
        }
        let re = Regex::new(r"^[A-Z]{2}$").unwrap();
        if !re.is_match(&value) {
            return Err(ErrorMsg("Language should be alpha-2 code (ISO 639-1)".to_string()))
        }
        Ok(Self(value))
    }
}

impl From<Language> for String {
    fn from(value: Language) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests_language {
    use super::*;

    #[test]
    fn test_language() {
        let language = Language::try_from("AA".to_owned());
        assert!(language.is_ok());
        let language = Language::try_from("A".to_owned());
        assert!(language.is_err());
        let language = Language::try_from("A2".to_owned());
        assert!(language.is_err());
        let language = Language::try_from("AA2".to_owned());
        assert!(language.is_err());
    }
}

#[derive(Clone)]
pub struct User {
    pub id: Option<Id>,
    pub email: Option<Email>,
    pub phone_number: Option<PhoneNumber>,
    pub password: Option<Password>,
    pub hashed_password: Option<String>,
    pub first_name: Option<FirstName>,
    pub last_name: Option<LastName>,
    pub birthday: Option<Birthday>,
    pub nationality: Nationality,
    pub languages: Option<Vec<Language>>,
    pub created_at:  Option<DateTime<Utc>>,
    pub updated_at:  Option<DateTime<Utc>>,
}