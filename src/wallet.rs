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
    let (external_descriptor, _ext_keymap) =
        match descriptor!(tr((seed.clone(), derivation_path_external.clone())))
            .unwrap()
            .into_wallet_descriptor(&secp, *network)
        {
            Ok((extended_descriptor, keymap)) => (extended_descriptor, keymap),
            Err(e) => panic!("Invalid external derivation path: {}", e),
        };
    let (internal_descriptor, _int_keymap) =
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
