use self::models::{AddNode, Node};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json
};
use lightining_node_api::*;

use diesel::data_types::PgNumeric;
use diesel::prelude::*;
use std::time::SystemTime;

// add a single node
async fn add_node(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    pub_key: String,
    alias: String,
    cap: PgNumeric,
    first_seen: SystemTime,
    updated_at: SystemTime,
) -> Result<Json<Node>, (StatusCode, String)> {
    use crate::schema::nodes;

    let conn = pool.get().await.map_err(internal_error)?;

    let nodes = AddNode {
        public_key: pub_key,
        alias: alias,
        capacity: cap,
        first_seen: first_seen,
        updated_at: updated_at,
    };

    let res = conn
        .interact(|conn| {
            diesel::insert_into(nodes::table)
                .values(nodes)
                .returning(Node::as_returning())
                .get_result(conn)
        })
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
}
