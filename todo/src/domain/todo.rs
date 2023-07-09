use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};


#[derive(Debug, PartialEq)]
#[derive(Deserialize, Serialize)]
pub enum Status {
    PENDING,
    STARTED,
    DONE,
    PAUSED,
    ABORTED,
}

impl TryFrom<i32> for Status {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Status::PENDING as i32 => Ok(Status::PENDING),
            x if x == Status::STARTED as i32 => Ok(Status::STARTED),
            x if x == Status::DONE as i32 => Ok(Status::DONE),
            x if x == Status::PAUSED as i32 => Ok(Status::PAUSED),
            x if x == Status::ABORTED as i32 => Ok(Status::ABORTED),
            _ => Err(()),
        }
    }
}

// impl Status {
//     pub fn from_u32(value: u8) -> Status {
//         match value {
//             0 => Status::PENDING,
//             1 => Status::STARTED,
//             2 => Status::DONE,
//             3 => Status::PAUSED,
//             4 => Status::ABORTED,
//             _ => panic!("Unknown value: {}", value),
//         }
//     }
// }

#[derive(Debug)]
pub struct Todo {
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub status: Status,
    pub create_date: Option<DateTime<Utc>>,
    pub done_date: Option<DateTime<Utc>>,
    pub deadline: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}
