#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vin {
	pub txid: String,
	pub sequence: i64,
	pub n: i64,
	pub addresses: Vec<String>,
	pub is_address: bool,
	pub value: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
	pub value: String,
	pub n: i64,
	//pub spent: bool,
	pub hex: String,
	pub addresses: Vec<String>,
	pub is_address: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainTransaction {
	pub txid: String,
	pub version: i64,
	pub vin: Vec<Vin>,
	pub vout: Vec<Vout>,
	pub block_hash: String,
	pub block_height: i64,
	pub confirmations: i64,
	pub block_time: i64,
	pub size: i64,
	pub vsize: i64,
	pub value: String,
	pub value_in: String,
	pub fees: String,
	pub hex: String,
}