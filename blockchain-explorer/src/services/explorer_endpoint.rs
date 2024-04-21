use dotenv;
use reqwest;
use tokio;
use serde_json::Result;
use crate::data::models::blockchain_status::BlockchainStatus;
use crate::data::models::blockchain_address::BlockchainAddress;
use crate::data::models::blockchain_transaction::BlockchainTransaction;

const HOST_ROOT: &str = "https://btcbook.nownodes.io/api/";

#[tokio::main]
pub async fn send_request(url: &str) -> String {
    let client = reqwest::Client::new();
    client
        .get(url)
        .header("api-key", dotenv::var("NOW_NODES_KEY").expect("Could not find the API Key"))
        .send()
        .await.expect("Failed to get response")
        .text()
        .await.expect("Failed to convert payload")
}

pub fn blockchain_status_request() -> BlockchainStatus {
    let response = send_request(&HOST_ROOT);
    serde_json::from_str(&response).expect("Failed to parse JSON")
}

pub fn blockchain_address_request(address: &str) -> BlockchainAddress {
    let response = send_request(&[HOST_ROOT, "v2/address/", &address].join(""));
    serde_json::from_str(&response).expect("Failed to parse JSON")
}

pub fn blockchain_transaction_request(transaction_hash: &str) -> BlockchainTransaction {
    let response = send_request(&[HOST_ROOT, "v2/tx/", &transaction_hash].join(""));
    serde_json::from_str(&response).expect("Failed to parse JSON")
}