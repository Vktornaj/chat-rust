use chrono::{Utc, Datelike, NaiveDate};
use common::domain::types::error::ErrorMsg;
use serde::{Deserialize, Serialize};


#[derive(PartialEq, Debug, Clone, PartialOrd, Serialize)]
pub struct Birthday(NaiveDate);

impl TryFrom<NaiveDate> for Birthday {
    type Error = ErrorMsg;

    fn try_from(value: NaiveDate) -> Result<Self, Self::Error> {
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

impl<'de> Deserialize<'de> for Birthday {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let birthday = NaiveDate::deserialize(deserializer)?;
        Self::try_from(birthday).map_err(serde::de::Error::custom)
    }
}

impl From<Birthday> for NaiveDate {
    fn from(birthday: Birthday) -> Self {
        birthday.0
    }
}

impl From<Birthday> for String {
    fn from(birthday: Birthday) -> Self {
        birthday.0.to_string()
    }
}   

// #[cfg(test)]
// mod tests_birthday {
//     use chrono::NaiveDate;

//     use super::*;

//     #[test]
//     fn test_birthday() {
//         let now = Utc::now();
//         // ok
//         let birthday = {
//             let dt = NaiveDate::from_ymd_opt(now.year() - 14, 1, 1)
//                 .unwrap()
//                 .and_hms_opt(0, 0, 0)
//                 .unwrap()
//                 .and_local_timezone(Utc)
//                 .unwrap();
//             Birthday::try_from(dt)
//         };
//         assert!(birthday.is_ok());
//         let birthday =  {
//             let dt = NaiveDate::from_ymd_opt(now.year() - 13, now.month(), now.day())
//                 .unwrap()
//                 .and_hms_opt(0, 0, 0)
//                 .unwrap()
//                 .and_local_timezone(Utc)
//                 .unwrap();
//             Birthday::try_from(dt)
//         };
//         assert!(birthday.is_ok());
//         // error
//         let birthday = {
//             let dt = NaiveDate::from_ymd_opt(now.year() - 13, now.month(), now.day() + 1)
//                 .unwrap()
//                 .and_hms_opt(0, 0, 0)
//                 .unwrap()
//                 .and_local_timezone(Utc)
//                 .unwrap();
//             Birthday::try_from(dt)
//         };
//         assert!(birthday.is_err());
//         let birthday = {
//             let dt = NaiveDate::from_ymd_opt(now.year() - 13, now.month() + 1, now.day())
//                 .unwrap()
//                 .and_hms_opt(0, 0, 0)
//                 .unwrap()
//                 .and_local_timezone(Utc)
//                 .unwrap();
//             Birthday::try_from(dt)
//         };
//         assert!(birthday.is_err());
//     }
// }
