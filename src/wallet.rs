use bdk::bitcoin::util::bip32::DerivationPath;
use bdk::bitcoin::{network::constants::Network, secp256k1::Secp256k1};
use bdk::database::MemoryDatabase;
use bdk::descriptor;
use bdk::descriptor::IntoWalletDescriptor;
use bdk::keys::bip39::Mnemonic;
use bdk::wallet::Wallet;
use std::str::FromStr;

pub fn create_derivation_path(input: &str) -> DerivationPath {
    match DerivationPath::from_str(input) {
        Ok(derivation_path) => derivation_path,
        Err(e) => panic!("Invalid derivation path: {}", e),
    }
}

pub fn create_wallet(
    seed: &Mnemonic,
    network: &Network,
    derivation_path_external: &DerivationPath,
    derivation_path_internal: &DerivationPath,
) -> Wallet<MemoryDatabase> {
    let secp = Secp256k1::new();

    // generate external and internal descriptor from mnemonic
    let (external_descriptor, ext_keymap) =
        match descriptor!(tr((seed.clone(), derivation_path_external.clone())))
            .unwrap()
            .into_wallet_descriptor(&secp, *network)
        {
            Ok((extended_descriptor, keymap)) => (extended_descriptor, keymap),
            Err(e) => panic!("Invalid external derivation path: {}", e),
        };
    let (internal_descriptor, int_keymap) =
        match descriptor!(tr((seed.clone(), derivation_path_internal.clone())))
            .unwrap()
            .into_wallet_descriptor(&secp, *network)
        {
            Ok((extended_descriptor, keymap)) => (extended_descriptor, keymap),
            Err(e) => panic!("Invalid internal derivation path: {}", e),
        };

    Wallet::new(
        external_descriptor,
        Some(internal_descriptor),
        *network,
        MemoryDatabase::new(),
    )
    .unwrap()
}

#[cfg(test)]
mod tests {
    use super::super::bip39::parse_mnemonic;
    use super::super::network::create_network;
    use super::*;
    use std::any::TypeId;

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
}
