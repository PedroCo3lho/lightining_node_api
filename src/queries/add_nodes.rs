use lightining_node_api::*;
use self::models::{AddNode, Node};

use std::time::SystemTime;
use diesel::prelude::*;
use diesel::data_types::PgNumeric;

// add a single node
fn add_node(
    conn: &mut PgConnection,
    pub_key: &str,
    alias: &str,
    cap: &PgNumeric,
    first_seen: &SystemTime,
    updated_at: &SystemTime,
) -> Node {
    use crate::schema::nodes;

    let nodes = AddNode {
        public_key: pub_key,
        alias: alias,
        capacity: cap,
        first_seen: first_seen,
        updated_at: updated_at,
    };

    diesel::insert_into(nodes::table)
        .values(nodes)
        .returning(Node::as_returning())
        .get_result(conn)
        .expect("error while adding a new node")
}