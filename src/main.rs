mod etl;
mod queries;

use axum::{Router, routing::get};
use diesel_async::{AsyncPgConnection, pooled_connection::AsyncDieselConnectionManager};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use etl::fetch_api::fetch_nodes;
use lightining_node_api::*;
use queries::{get_nodes::get_nodes, add_nodes::add_node};
use tokio::time::interval;
use std::{env, net::SocketAddr, time::Duration};

pub type Pool = bb8::Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // set up connection pool
    let manager = deadpool_diesel::postgres::Manager::new(db_url, deadpool_diesel::Runtime::Tokio1);
    let pool: deadpool_diesel::Pool<deadpool_diesel::Manager<diesel::PgConnection>> = deadpool_diesel::postgres::Pool::builder(manager)
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

    seeder(&pool).await;

    // tokio::spawn(async move {
    //     let mut interval = interval(Duration::from_secs(60*5));
    //     loop {
    //         interval.tick().await;
    //     }
    // });

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

async fn seeder(pool: &deadpool_diesel::Pool<deadpool_diesel::Manager<diesel::PgConnection>>) {
    use axum::extract::State;

    let state = State(pool.clone());
    match get_nodes(state.clone()).await {
        Ok(nodes) if !nodes.0.is_empty() => {
            println!("Database already populated!");
            return ;
        }
        _ => {
            println!("Populating database...");
            let nodes = fetch_nodes().await;
            for node in nodes {
                let res = add_node(state.clone(), node.public_key, node.alias, node.capacity, node.first_seen, node.updated_at).await;
                println!("{:?}", res);
            }
        }
    }
}
