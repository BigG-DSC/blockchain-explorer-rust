use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blockbook {
    pub coin: String,
    pub host: String,
    pub version: String,
    pub git_commit: String,
    pub build_time: String,
    pub sync_mode: bool,
    pub initial_sync: bool,
    pub in_sync: bool,
    pub best_height: i64,
    pub last_block_time: String,
    pub in_sync_mempool: bool,
    pub last_mempool_time: String,
    pub mempool_size: i64,
    pub decimals: i64,
    pub db_size: i64,
    pub has_fiat_rates: bool,
    pub current_fiat_rates_time: String,
    pub about: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Backend {
    pub chain: String,
    pub blocks: i64,
    pub headers: i64,
    pub best_block_hash: String,
    pub difficulty: String,
    pub size_on_disk: i64,
    pub version: String,
    pub subversion: String,
    pub protocol_version: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainStatus {
    pub blockbook: Blockbook,
    pub backend: Backend,
}