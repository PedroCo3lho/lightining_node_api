
use self::models::Node;

use axum::{extract::State, Json, http::StatusCode};
use diesel::prelude::*;
use lightining_node_api::*;


// GET all nodes from the database
pub async fn get_nodes(State(pool): State<deadpool_diesel::postgres::Pool>,) ->  Result<Json<Vec<Node>>, (StatusCode, String)> {
    use self::schema::nodes::dsl::*;

    let conn = pool.get().await.map_err(internal_error)?;

    // let connection = &mut establish_connection();

    let res = conn
        .interact(|conn| nodes.select(Node::as_select()).load(conn))
        .await
        .map_err(internal_error)?
        .map_err(internal_error)?;
    Ok(Json(res))
    // println!("Displaying {} nodes", results.len());
    // let mut new_nodes: Vec<Node> = vec![];
    // for node in results {
    //     new_nodes.push(Node {
    //         public_key: node.public_key,
    //         alias: node.alias,
    //         capacity: node.capacity,
    //         first_seen: node.first_seen,
    //         updated_at: node.updated_at,
    //     });
    // }
}