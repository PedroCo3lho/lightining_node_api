use lightining_node_api::*;
use self::models::Node;
use chrono::{DateTime, Utc};
use diesel::data_types::PgNumeric;
use reqwest::Request;
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, UNIX_EPOCH};

pub async fn fetch_nodes() -> Vec<Node> {
    let client = Arc::new(reqwest::Client::new()); 
    let request_url = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

    let req: Request = client.get(request_url).build().unwrap();
    let resp = client.execute(req).await.unwrap();

    let response: serde_json::Value = resp.json().await.unwrap();

    process_nodes(&response)
}

fn process_nodes(data: &Value) -> Vec<Node> {
    // I'll need to type the response to interate with each one to convert values and select values
    let mut processed_nodes: Vec<Node> = vec![];
    if let Value::Array(nodes_array) = data {
        for node in nodes_array {
            if let Value::Object(node_info) = node {
                let pub_key: String = node_info
                    .get_key_value("publicKey")
                    .and_then(|(_, value)| value.as_str().map(|s| s.to_string()))
                    .unwrap_or_default();
                let alias = node_info
                    .get_key_value("alias")
                    .and_then(|(_, value)| value.as_str().map(|s| s.to_string()))
                    .unwrap_or_default();
                let sats_capaciticy = node_info
                    .get_key_value("capacity")
                    .and_then(|(_, value)| value.as_u64())
                    .unwrap_or_default();
                let unix_first_seen = node_info
                    .get_key_value("firstSeen")
                    .and_then(|(_, value)| value.as_u64())
                    .unwrap_or_default();
                let unix_update_at = node_info
                    .get_key_value("updatedAt")
                    .and_then(|(_, value)| value.as_u64())
                    .unwrap_or_default();

                let btc_capacity: i64 = sats_capaciticy as i64 / 100_000_000;
                let dt_first_seen = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(unix_first_seen));
                let dt_updated_at = DateTime::<Utc>::from(UNIX_EPOCH + Duration::from_secs(unix_update_at));

                let capacity_pg_numeric = PgNumeric::Positive { weight: (0), scale: (8), digits: (vec![btc_capacity.try_into().unwrap()]) };

                processed_nodes.push(Node { public_key: (pub_key), alias: (alias), capacity: (capacity_pg_numeric), first_seen: (dt_first_seen.into()), updated_at: (dt_updated_at.into()) });
            }
        }
    }
    processed_nodes
}
