use axum::{Router, extract::State, routing::get, Json};
use reqwest::Request;
use std::{net::SocketAddr, time::SystemTime};
use std::sync::Arc;
use self::models::*;
use diesel::{data_types::PgNumeric, prelude::*};
use lightining_node_api::*;

#[tokio::main]
async fn main() {
    let client = Arc::new(reqwest::Client::new()); // Arc is required to set client on app state

    let app = Router::new().route("/", get(fetch_nodes)).with_state(client);

    println!("Running on http://localhost:3000");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn fetch_nodes(State(client): State<Arc<reqwest::Client>>) -> Json<serde_json::Value> {
    let request_url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

    let req: Request = client.get(request_url).build().unwrap();
    let resp = client.execute(req).await.unwrap();

    let response: serde_json::Value = resp.json().await.unwrap();

    Json(response)
}

fn proccess_nodes(data: Json<serde_json::Value>){
    // I'll need to type the response to interate with each one to convert values and select values 
}

// GET all nodes from the database
async fn get_nodes() -> Json<Vec<Node>> {
    use self::schema::nodes::dsl::*;

    let connection = &mut establish_connection();
    let results = nodes
        .select(Node::as_select())
        .load(connection)
        .expect("Error fetchin the Nodes");

    println!("Displaying {} nodes", results.len());
    let mut new_nodes: Vec<Node> = vec![];
    for node in results {
        new_nodes.push(Node{public_key: node.public_key, alias: node.alias, capacity: node.capacity, first_seen: node.first_seen, updated_at: node.updated_at});
    }
    Json(new_nodes)
}

// POST nodes to the databse, intend to be ran only one time, the updates will work mofifing the alread exitents nodes with a condicional update based on last-update
use self::models::{AddNode, Node};

fn add_node(conn: &mut PgConnection, pub_key: &str, alias: &str, cap: &PgNumeric, first_seen: &SystemTime, updated_at: &SystemTime) -> Node{
    use crate::schema::nodes;

    let nodes = AddNode {public_key: pub_key, alias: alias, capacity: cap, first_seen: first_seen, updated_at: updated_at};

    diesel::insert_into(nodes::table)
        .values(nodes)
        .returning(Node::as_returning())
        .get_result(conn)
        .expect("error while adding a new node")
}