#[macro_use]
extern crate serde_derive;

mod data {
    pub mod models {
        pub mod blockchain_status;
        pub mod blockchain_address;
        pub mod blockchain_transaction;
    }
}

mod services{
    pub mod explorer_endpoint;
}

use {
    crate::data::models::blockchain_status::BlockchainStatus,
    crate::data::models::blockchain_address::BlockchainAddress,
    crate::data::models::blockchain_transaction::BlockchainTransaction,
    dotenv,
    std::{io, thread, time},
};

fn main() {
    let blockchain_status: BlockchainStatus = services::explorer_endpoint::blockchain_status_request();
    println!("Querying Status {} - Chain {}", &blockchain_status.blockbook.coin, &blockchain_status.backend.chain);

    let address = dotenv::var("ADDRESS_TO_FETCH").expect("Could not find the API Key");
    let blockchain_address_info: BlockchainAddress = services::explorer_endpoint::blockchain_address_request(&address);
    println!("Transactions for Address {}", &blockchain_address_info.address);
    println!("You have a total of {} transactions", &blockchain_address_info.txids.len());
    println!("Do you want to show the transactions?[y/n]");

    let mut command = String::new();
    io::stdin().read_line(&mut command);

    if command.trim().eq("y") {
        println!("Showing transactions...");
        thread::sleep(time::Duration::from_millis(1000));

        let mut balance: i64 = 0;
        for tx_id in &blockchain_address_info.txids {
            println!("---------------------------------------------------------");
            println!("Transaction ID {}", &tx_id);
            let mut subtotal_vin: i64 = 0;
            let mut subtotal_vout: i64 = 0;
            let blockchain_transaction: BlockchainTransaction = services::explorer_endpoint::blockchain_transaction_request(&tx_id);

            for tx in &blockchain_transaction.vin {
                if tx.addresses.contains(&address) {
                    subtotal_vin += tx.value.parse::<i64>().unwrap();
                }
            }

            for tx in &blockchain_transaction.vout {
                if tx.addresses.contains(&address) {
                    subtotal_vout += tx.value.parse::<i64>().unwrap();
                }
            }

            balance += &subtotal_vout - &subtotal_vin;

            println!("TX ID:           {}", &blockchain_transaction.txid);
            println!("SATOSHIS IN:     {}", &subtotal_vout);
            println!("SATOSHIS OUT:    {}", &subtotal_vin);
            println!("BALANCE:         {}", &balance);
            println!("---------------------------------------------------------");
        }

        println!("Current Balance:     {}", &balance);
        println!("         In BTC:     {}", balance as f64 * 0.00000001);
    }
}
