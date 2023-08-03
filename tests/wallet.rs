use bdk::bitcoin::util::bip32::DerivationPath;
use bdk::database::MemoryDatabase;
use bdk::wallet::Wallet;
use std::any::TypeId;
use sweepr::bip39::parse_mnemonic;
use sweepr::network::create_network;
use sweepr::wallet::{create_derivation_path, create_wallet};

#[cfg(feature = "test-sync")]
use sweepr::network::create_blockchain;
#[cfg(feature = "test-sync")]
use sweepr::sync::{check_balance, sync_wallet};

fn is_derivationpath<T: ?Sized + 'static>(_s: &T) -> bool {
    TypeId::of::<DerivationPath>() == TypeId::of::<T>()
}
fn is_wallet<T: ?Sized + 'static>(_s: &T) -> bool {
    TypeId::of::<Wallet<MemoryDatabase>>() == TypeId::of::<T>()
}
#[cfg(feature = "test-sync")]
fn is_bool<T: ?Sized + 'static>(_s: &T) -> bool {
    TypeId::of::<bool>() == TypeId::of::<T>()
}

#[test]
fn test_create_derivation_path() {
    assert!(is_derivationpath(&create_derivation_path("m/44'/0'/0'/0")));
    assert!(is_derivationpath(&create_derivation_path("m/44'/0'/0'/1")));
    assert!(is_derivationpath(&create_derivation_path("m/44h/0h/0h/0")));
    assert!(is_derivationpath(&create_derivation_path("m/49'/0'/0'/0")));
    assert!(is_derivationpath(&create_derivation_path("m/49'/0'/0'/1")));
    assert!(is_derivationpath(&create_derivation_path("m/49h/0h/0h/0")));
    assert!(is_derivationpath(&create_derivation_path("m/84'/0'/0'/0")));
    assert!(is_derivationpath(&create_derivation_path("m/84'/0'/0'/1")));
    assert!(is_derivationpath(&create_derivation_path("m/84h/0h/0h/0")));
}

#[test]
#[should_panic]
fn test_invalid_derivation_path() {
    create_derivation_path("Hello!");
}

#[test]
fn test_create_wallet() {
    let mnemonic_12 = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon cactus";
    let mnemonic_24 = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
    let parsed_mnemonic_12 = parse_mnemonic(mnemonic_12);
    let parsed_mnemonic_24 = parse_mnemonic(mnemonic_24);

    let mainnet = create_network("mainnet");
    let testnet = create_network("testnet");
    let regtest = create_network("regtest");

    let derivation_path_external = create_derivation_path("m/84'/0'/0'/0");
    let derivation_path_internal = create_derivation_path("m/84'/0'/0'/1");

    let wallet_mainnet_12 = create_wallet(
        &parsed_mnemonic_12,
        &mainnet,
        &derivation_path_external,
        &derivation_path_internal,
    );
    let wallet_mainnet_24 = create_wallet(
        &parsed_mnemonic_24,
        &mainnet,
        &derivation_path_external,
        &derivation_path_internal,
    );
    let wallet_testnet_12 = create_wallet(
        &parsed_mnemonic_12,
        &testnet,
        &derivation_path_external,
        &derivation_path_internal,
    );
    let wallet_testnet_24 = create_wallet(
        &parsed_mnemonic_24,
        &testnet,
        &derivation_path_external,
        &derivation_path_internal,
    );
    let wallet_regtest_12 = create_wallet(
        &parsed_mnemonic_12,
        &regtest,
        &derivation_path_external,
        &derivation_path_internal,
    );
    let wallet_regtest_24 = create_wallet(
        &parsed_mnemonic_24,
        &regtest,
        &derivation_path_external,
        &derivation_path_internal,
    );

    assert!(is_wallet(&wallet_mainnet_12));
    assert!(is_wallet(&wallet_mainnet_24));
    assert!(is_wallet(&wallet_testnet_12));
    assert!(is_wallet(&wallet_testnet_24));
    assert!(is_wallet(&wallet_regtest_12));
    assert!(is_wallet(&wallet_regtest_24));

    assert_eq!(wallet_mainnet_12.network(), mainnet);
    assert_eq!(wallet_mainnet_24.network(), mainnet);
    assert_eq!(wallet_testnet_12.network(), testnet);
    assert_eq!(wallet_testnet_24.network(), testnet);
    assert_eq!(wallet_regtest_12.network(), regtest);
    assert_eq!(wallet_regtest_24.network(), regtest);
}

#[tokio::test]
#[cfg(feature = "test-sync")]
async fn test_sync_wallet_and_check_balance() {
    let mnemonic_24 = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
    let parsed_mnemonic_24 = parse_mnemonic(mnemonic_24);

    let mainnet = create_network("mainnet");
    let testnet = create_network("testnet");

    let derivation_path_external = create_derivation_path("m/84'/0'/0'/0");
    let derivation_path_internal = create_derivation_path("m/84'/0'/0'/1");

    let wallet_mainnet_24 = create_wallet(
        &parsed_mnemonic_24,
        &mainnet,
        &derivation_path_external,
        &derivation_path_internal,
    );
    let wallet_testnet_24 = create_wallet(
        &parsed_mnemonic_24,
        &testnet,
        &derivation_path_external,
        &derivation_path_internal,
    );

    let esplora_mainnet = create_blockchain("https://mempool.space/api", None);
    let esplora_testnet = create_blockchain("https://mempool.space/testnet/api", None);

    sync_wallet(&wallet_mainnet_24, &esplora_mainnet).await;
    sync_wallet(&wallet_testnet_24, &esplora_testnet).await;

    let balance_mainnet = check_balance(&wallet_mainnet_24);
    let balance_testnet = check_balance(&wallet_testnet_24);

    assert!(is_bool(&balance_mainnet));
    assert!(is_bool(&balance_testnet));
}
