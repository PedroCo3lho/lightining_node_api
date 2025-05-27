mod queries;

use axum::{Router, routing::get};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lightining_node_api::*;
use queries::get_nodes::get_nodes;
use std::net::SocketAddr;
use diesel_async::{
    pooled_connection::AsyncDieselConnectionManager, AsyncPgConnection
};

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL").unwrap();

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