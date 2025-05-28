use crate::etl::fetch_api::fetch_nodes;

use axum::{extract::State, http::StatusCode};
use diesel::prelude::*;
use lightining_node_api::{
    schema::nodes::{capacity, first_seen, public_key, updated_at},
    *,
};

pub async fn update_node(
    State(pool): State<deadpool_diesel::postgres::Pool>,
) -> Result<(), StatusCode> {
    use crate::schema::nodes;

    let conn = pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let nodes = fetch_nodes().await;

    let _res = conn
        .interact(move |conn| {
            let mut updated = 0;
            for node in &nodes {
                // Update each node based on public_key
                updated += diesel::update(nodes::table.filter(public_key.eq(&node.public_key)))
                    .set((capacity.eq(node.capacity), first_seen.eq(node.first_seen), updated_at.eq(node.updated_at)))
                    .execute(conn)?;
            }
            println!("{updated} nodes updated.");
            Ok::<_, diesel::result::Error>(updated)
        })
        .await
        .map_err(internal_error);

    Ok(())
}
