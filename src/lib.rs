pub mod models;
pub mod schema;

use diesel::{data_types::PgNumeric, dsl::now, prelude::*};
use dotenvy::{self, dotenv};
use std::env;
use std::time::SystemTime;


pub fn establish_connection() -> PgConnection{
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// use self::models::{AddNode, Node};

// fn add_node(conn: &mut PgConnection, pub_key: &str, alias: &str, cap: &PgNumeric, first_seen: &SystemTime) -> Node{
//     use crate::schema::nodes;

//     let nodes = AddNode {public_key: pub_key, alias: alias, capacity: cap, first_seen: first_seen, updated_at: &SystemTime::now()};

//     diesel::insert_into(nodes::table)
//         .values(nodes)
//         .returning(Node::as_returning())
//         .get_result(conn)
//         .expect("error while adding a new node")
// }