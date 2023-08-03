use bdk::{bitcoin::network::constants::Network, blockchain::esplora::EsploraBlockchain};
use core::any::TypeId;
use sweepr::network::{create_blockchain, create_network};

fn is_esplorablockchain<T: ?Sized + 'static>(_s: &T) -> bool {
    TypeId::of::<EsploraBlockchain>() == TypeId::of::<T>()
}

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

#[test]
fn test_create_blockchain() {
    assert!(is_esplorablockchain(&create_blockchain(
        "https://mempool.space/api",
        None
    )));
    assert!(is_esplorablockchain(&create_blockchain(
        "https://mempool.space/testnet/api",
        None
    )));
    assert!(is_esplorablockchain(&create_blockchain(
        "localhost:3000/api",
        None
    )));
    assert!(is_esplorablockchain(&create_blockchain(
        "https://mempool.space/api",
        Some(10)
    )));
    assert!(is_esplorablockchain(&create_blockchain(
        "https://mempool.space/testnet/api",
        Some(10)
    )));
    assert!(is_esplorablockchain(&create_blockchain(
        "localhost:3000/api",
        Some(10)
    )));
}
