# Bitcoin Explorer Rust

Bitcoin Explorer Rust is a backend application written in Rust that scrapes the Bitcoin blockchain to provide comprehensive information about Bitcoin transactions and blocks.

## Features

- Fetch detailed information about Bitcoin transactions.
- Explore Bitcoin addresses to view transaction history.

## Installation

To install and run, follow these steps:

1. **Clone the Repository**: 
   ```bash
   git clone https://github.com/yourusername/bitcoin-explorer-rust.git
   cd bitcoin-explorer-rust
   cargo build
   cargo run
   ```

2. **Set the .env file**: 
   ```bash
    API_ENDPOINT="https://btcbook.nownodes.io/api/"
    NOW_NODES_KEY="YOUR-API-KEY"
   ```

## API Endpoints

- `/blockchain-status`: Retrieve details about the current status of the Bitcoin blockchain.
- `/blockchain-address/{address}`: Retrieve details about a specific Bitcoin transaction.
- `/blockchain-transaction/{transaction_id}`: Explore transaction history for a Bitcoin address.

## Contributing
Contributions are welcome! If you want to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Commit your changes and push the branch to your fork.
4. Open a pull request against the main repository.

Please ensure that your code follows the project's coding style and conventions.

## License
This project is licensed under the MIT License.