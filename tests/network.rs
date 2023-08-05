use bdk::bitcoin::network::constants::Network;
use bdk_esplora::esplora_client::AsyncClient;
use core::any::TypeId;
use sweepr::network::{create_client, create_network};

fn is_esplorablockchain<T: ?Sized + 'static>(_s: &T) -> bool {
    TypeId::of::<AsyncClient>() == TypeId::of::<T>()
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
    assert!(is_esplorablockchain(&create_client(
        "https://mempool.space/api",
    )));
    assert!(is_esplorablockchain(&create_client(
        "https://mempool.space/testnet/api",
    )));
    assert!(is_esplorablockchain(&create_client("localhost:3000/api",)));
}
