use self::models::Node;

use axum::{Json, extract::State, http::StatusCode};
use diesel::prelude::*;
use lightining_node_api::*;

// GET all nodes from the database
pub async fn get_nodes(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<Json<Vec<Node>>, StatusCode> {
    use self::schema::nodes::dsl::*;

    let conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let res = conn
        .interact(|conn| nodes.select(Node::as_select()).load(conn))
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(res))
}
