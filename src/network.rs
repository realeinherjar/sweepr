use bdk::blockchain::ElectrumBlockchain;
use bdk::electrum_client::Client;
use bitcoin::network::constants::Network;

fn create_con(server: &str) -> Client {
    match Client::new(server) {
        Ok(con) => con,
        Err(e) => panic!("Error creating connection: {}", e),
    }
}
fn create_blockchain(server: &str) -> ElectrumBlockchain {
    let client = create_con(server);
    ElectrumBlockchain::from(client)
}

fn create_network(network: &str) -> Network {
    let network = network.to_lowercase();
    match network.as_str() {
        "mainnet" => Network::Bitcoin,
        "testnet" => Network::Testnet,
        "regtest" => Network::Regtest,
        _ => panic!("Invalid network"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network() {
        assert_eq!(create_network("mainnet"), Network::Bitcoin);
        assert_eq!(create_network("testnet"), Network::Testnet);
        assert_eq!(create_network("regtest"), Network::Regtest);
        assert_eq!(create_network("Mainnet"), Network::Bitcoin);
    }

    #[test]
    #[should_panic]
    fn test_invalid_network() {
        create_network("invalid");
    }
}
