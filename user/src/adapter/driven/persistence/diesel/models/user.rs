use diesel::{prelude::*};

use super::super::schema::_user;
use crate::domain::user::User as UserDomain;


#[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(primary_key(id))]
#[diesel(table_name = _user)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
}

impl User {
    pub fn to_user_domain(self) -> UserDomain {
        UserDomain {
            id: Some(self.id),
            username: self.username,
            first_name: self.first_name,
            last_name: self.last_name,
            password: self.password,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = _user)]
pub struct NewUser {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
}

impl NewUser {
    pub fn from_user_domain(user: UserDomain) -> Self {
        NewUser {
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            password: user.password,
        }
    }
}