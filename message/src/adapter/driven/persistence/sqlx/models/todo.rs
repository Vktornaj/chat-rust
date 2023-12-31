// use chrono::{DateTime, Utc};
// use uuid::Uuid;

// use crate::domain::message::{Todo as TodoDomain, Status};


// pub struct Todo {
//     pub id: i32,
//     pub user_id: Uuid,
//     pub title: String,
//     pub description: Option<String>,
//     pub status: i32,
//     pub create_date: DateTime<Utc>,
//     pub done_date: Option<DateTime<Utc>>,
//     pub deadline: Option<DateTime<Utc>>,
// }

// impl Todo {
//     pub fn to_domain_todo(self, tags: Option<Vec<String>>) -> TodoDomain {
//         TodoDomain {
//             id: Some(self.id),
//             user_id: Some(self.user_id),
//             title: self.title,
//             description: self.description,
//             status: (self.status as i32).try_into().unwrap_or(Status::ABORTED),
//             create_date: Some(self.create_date),
//             done_date: self.done_date,
//             deadline: self.deadline,
//             tags,
//         }
//     }
// }