use diesel::{data_types::PgNumeric, prelude::*};
use std::time::SystemTime;
use crate::schema::nodes;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::nodes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Node {
    //pub id: i32,
    pub public_key: String,
    pub alias: String,
    pub capacity: PgNumeric,
    pub first_seen: SystemTime,
    pub updated_at: SystemTime, // to compare if the new fetch is newer
}

// generated because of PgNumeric
impl serde::Serialize for Node {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        todo!()
    }
}

#[derive(Insertable)]
#[diesel(table_name = nodes)]
pub struct AddNode<'a> {
    pub public_key: &'a str,
    pub alias: &'a str,
    pub capacity: &'a PgNumeric,
    pub first_seen: &'a SystemTime,
    pub updated_at: &'a SystemTime,
}