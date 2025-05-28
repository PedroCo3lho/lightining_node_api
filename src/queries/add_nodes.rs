use self::models::{AddNode, Node};
use axum::{extract::State, http::StatusCode, response::Json};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use lightining_node_api::*;

// add a single node
async fn add_node(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    pub_key: String,
    alias: String,
    cap: f64,
    first_seen: NaiveDateTime,
    updated_at: NaiveDateTime,
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
