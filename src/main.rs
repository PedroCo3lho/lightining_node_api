mod etl;
mod queries;

use axum::{Router, routing::get};
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use etl::fetch_api::fetch_nodes;
use lightining_node_api::*;
use queries::get_nodes::get_nodes;
use std::{env, net::SocketAddr};

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool = deadpool_diesel::postgres::Pool::builder(manager)
        .build()
        .unwrap();

    // run the migrations on server startup
    {
        let conn = pool.get().await.unwrap();
        conn.interact(|conn| conn.run_pending_migrations(MIGRATIONS).map(|_| ()))
            .await
            .unwrap()
            .unwrap();
    }

    let fnodes = fetch_nodes().await;

    println!("{:?}", fnodes);

    // build our application with some routes
    let app = Router::new()
        .route("/nodes", get(get_nodes))
        .with_state(pool);

    println!("Running on http://localhost:3000");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
