mod utils;

use bdk::{
    bitcoin::{
        network::constants::Network,
        util::{bip32::DerivationPath, psbt::PartiallySignedTransaction},
    },
    wallet::{ChangeSet, Wallet},
};
use bdk_file_store::Store;
use std::any::TypeId;
use sweepr::{
    bip39::parse_mnemonic,
    network::{create_client, create_network},
    wallet::{
        create_address, create_derivation_path, create_derivation_paths_with_last_index,
        create_signed_transaction, create_wallet, get_fee_estimates,
    },
};
use utils::get_funded_wallet_with_change;

#[cfg(feature = "test-sync")]
use sweepr::wallet::{check_balance, sync_wallet};

fn is_derivationpath<T: ?Sized + 'static>(_s: &T) -> bool {
    TypeId::of::<DerivationPath>() == TypeId::of::<T>()
}
fn is_wallet<T: ?Sized + 'static>(_s: &T) -> bool {
    TypeId::of::<Wallet<Store<ChangeSet>>>() == TypeId::of::<T>()
}
#[cfg(feature = "test-sync")]
fn is_bool<T: ?Sized + 'static>(_s: &T) -> bool {
    TypeId::of::<bool>() == TypeId::of::<T>()
}

fn is_psbt<T: ?Sized + 'static>(_s: &T) -> bool {
    TypeId::of::<PartiallySignedTransaction>() == TypeId::of::<T>()
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
fn test_create_derivation_paths_with_last_index() {
    let (derivation_path_external, derivation_path_internal) =
        create_derivation_paths_with_last_index("m/44'/0'/0'/");
    assert!(is_derivationpath(&derivation_path_external));
    assert!(is_derivationpath(&derivation_path_internal));
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
        parsed_mnemonic_12.clone(),
        mainnet.clone(),
        derivation_path_external.clone(),
        derivation_path_internal.clone(),
    );
    let wallet_mainnet_24 = create_wallet(
        parsed_mnemonic_24.clone(),
        mainnet.clone(),
        derivation_path_external.clone(),
        derivation_path_internal.clone(),
    );
    let wallet_testnet_12 = create_wallet(
        parsed_mnemonic_12.clone(),
        testnet.clone(),
        derivation_path_external.clone(),
        derivation_path_internal.clone(),
    );
    let wallet_testnet_24 = create_wallet(
        parsed_mnemonic_24.clone(),
        testnet.clone(),
        derivation_path_external.clone(),
        derivation_path_internal.clone(),
    );
    let wallet_regtest_12 = create_wallet(
        parsed_mnemonic_12.clone(),
        regtest.clone(),
        derivation_path_external.clone(),
        derivation_path_internal.clone(),
    );
    let wallet_regtest_24 = create_wallet(
        parsed_mnemonic_24.clone(),
        regtest.clone(),
        derivation_path_external.clone(),
        derivation_path_internal.clone(),
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

    let mut wallet_mainnet_24 = create_wallet(
        parsed_mnemonic_24.clone(),
        mainnet.clone(),
        derivation_path_external.clone(),
        derivation_path_internal.clone(),
    );
    let mut wallet_testnet_24 = create_wallet(
        parsed_mnemonic_24.clone(),
        testnet.clone(),
        derivation_path_external.clone(),
        derivation_path_internal.clone(),
    );

    let esplora_mainnet = create_client("https://mempool.space/api");
    let esplora_testnet = create_client("https://mempool.space/testnet/api");

    sync_wallet(&mut wallet_mainnet_24, &esplora_mainnet).await;
    sync_wallet(&mut wallet_testnet_24, &esplora_testnet).await;

    let balance_mainnet = check_balance(&wallet_mainnet_24);
    let balance_testnet = check_balance(&wallet_testnet_24);

    assert!(is_bool(&balance_mainnet));
    assert!(is_bool(&balance_testnet));
}

#[test]
fn test_create_address() {
    let address_mainnet = create_address("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq");
    let address_testnet = create_address("mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn");
    assert!(address_mainnet.is_valid_for_network(Network::Bitcoin));
    assert!(address_testnet.is_valid_for_network(Network::Testnet));
}

#[tokio::test]
async fn test_get_fee_estimates() {
    let esplora_mainnet = create_client("https://mempool.space/api");
    let esplora_testnet = create_client("https://mempool.space/testnet/api");
    let fee_estimates_mainnet = get_fee_estimates(&esplora_mainnet, None).await;
    let fee_estimates_testnet = get_fee_estimates(&esplora_testnet, None).await;
    assert!(fee_estimates_mainnet > 0.0);
    assert!(fee_estimates_testnet > 0.0);
}

#[tokio::test]
async fn test_create_signed_transaction() {
    let mnemonic_24 = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
    let parsed_mnemonic_24 = parse_mnemonic(mnemonic_24);

    let derivation_path_external = create_derivation_path("m/84'/0'/0'/0");
    let derivation_path_internal = create_derivation_path("m/84'/0'/0'/1");

    let (mut wallet, _txid) = get_funded_wallet_with_change(
        parsed_mnemonic_24,
        derivation_path_external,
        derivation_path_internal,
    );

    let address_mainnet = create_address("bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq");
    let address_testnet = create_address("mipcBbFg9gMiCh81Kj8tqqdgoZub1ZJRfn");

    let esplora_mainnet = create_client("https://mempool.space/api");
    let esplora_testnet = create_client("https://mempool.space/testnet/api");

    let psbt_mainnet =
        create_signed_transaction(&mut wallet, address_mainnet, &esplora_mainnet).await;
    let psbt_testnet =
        create_signed_transaction(&mut wallet, address_testnet, &esplora_testnet).await;

    assert!(is_psbt(&psbt_mainnet));
    assert!(is_psbt(&psbt_testnet));
}
