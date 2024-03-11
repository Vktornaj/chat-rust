use common::domain::types::id::Id;
use serde::{Deserialize, Serialize};
use crate::domain::types::alias::Alias;


#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewContactJson {
    pub id: Id,
    pub alias: Option<Alias>,
    pub is_blocked: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateContactJson {
    pub id: Id,
    pub alias: Option<Option<Alias>>,
    pub is_blocked: Option<bool>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdJson {
    pub id: Id,
}