use bdk::bitcoin::network::constants::Network;
use sweepr::network::create_network;

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
