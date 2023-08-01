use bdk::bitcoin::network::constants::Network;
use bdk::blockchain::esplora::EsploraBlockchain;

pub fn create_blockchain(url: &str, stop_gap: Option<usize>) -> EsploraBlockchain {
    // TODO: 20 stop_gap is hardcoded for now
    EsploraBlockchain::new(url, stop_gap.unwrap_or(20))
}

pub fn create_network(network: &str) -> Network {
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
