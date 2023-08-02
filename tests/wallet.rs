use bdk::bitcoin::util::bip32::DerivationPath;
use bdk::database::MemoryDatabase;
use bdk::wallet::Wallet;
use std::any::TypeId;
use sweepr::bip39::parse_mnemonic;
use sweepr::wallet::create_wallet;
use sweepr::{network::create_network, wallet::create_derivation_path};

fn is_derivationpath<T: ?Sized + 'static>(_s: &T) -> bool {
    TypeId::of::<DerivationPath>() == TypeId::of::<T>()
}
fn is_wallet<T: ?Sized + 'static>(_s: &T) -> bool {
    TypeId::of::<Wallet<MemoryDatabase>>() == TypeId::of::<T>()
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
}
