use self::models::Node;

use axum::{Json, extract::State, http::StatusCode, extract::Query};
use diesel::prelude::*;
use lightining_node_api::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Order {
    order: Option<String>
}

// GET all nodes from the database
pub async fn get_nodes(
    State(pool): State<deadpool_diesel::postgres::Pool>,
    order: Query<Order>
) -> Result<Json<Vec<Node>>, StatusCode> {
    use self::schema::nodes::dsl::*;

    let order = order.0.order;
    
    let conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if order.is_some_and(|order| order == "capacity") {
        let res = conn
            .interact(|conn| nodes
            .select(Node::as_select()).order(capacity.desc()).load(conn))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
        Ok(Json(res))
    }
    else{
        let res = conn
            .interact(|conn| nodes.select(Node::as_select()).load(conn))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
        Ok(Json(res))
    }

}
