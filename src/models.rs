use crate::schema::nodes;
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Debug, serde::Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Node {
    //pub id: i32,
    pub public_key: String,
    pub alias: String,
    pub capacity: f64,
    pub first_seen: NaiveDateTime,
    pub updated_at: NaiveDateTime, // to compare if the new fetch is newer
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = nodes)]
pub struct AddNode {
    pub public_key: String,
    pub alias: String,
    pub capacity: f64,
    pub first_seen: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}