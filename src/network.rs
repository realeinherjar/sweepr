use bdk::bitcoin::network::constants::Network;
use bdk_esplora::esplora_client::{AsyncClient, Builder};

/// Creates a client from a url.
pub fn create_client(url: &str) -> AsyncClient {
    match Builder::new(url).build_async() {
        Ok(client) => client,
        Err(e) => panic!("Invalid esplora url: {}", e),
    }
}

/// Creates a network from a string.
pub fn create_network(network: &str) -> Network {
    let network = network.to_lowercase();
    match network.as_str() {
        "mainnet" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        "regtest" => Network::Regtest,
        _ => panic!("Invalid network"),
    }
}
