use bdk::{
    bitcoin::{
        network::constants::Network,
        psbt::PartiallySignedTransaction,
        secp256k1::Secp256k1,
        util::{address::Address, bip32::DerivationPath},
    },
    descriptor,
    descriptor::IntoWalletDescriptor,
    keys::bip39::Mnemonic,
    wallet::{ChangeSet, Wallet},
    Error, FeeRate, TransactionDetails,
};
use bdk_esplora::EsploraAsyncExt;
use bdk_file_store::Store;
use std::env::temp_dir;
use std::str::FromStr;

const DB_MAGIC: &str = "sweepr";
const STOP_GAP: usize = 5;
const PARALLEL_REQUESTS: usize = 5;

pub fn create_derivation_path(input: &str) -> DerivationPath {
    match DerivationPath::from_str(input) {
        Ok(derivation_path) => derivation_path,
        Err(e) => panic!("Invalid derivation path: {}", e),
    }
}

pub fn create_wallet<'a>(
    seed: Mnemonic,
    network: Network,
    derivation_path_external: DerivationPath,
    derivation_path_internal: DerivationPath,
) -> Wallet<Store<'a, ChangeSet>> {
    let db_path = temp_dir().join("sweepr");
    let db = Store::<ChangeSet>::new_from_path(DB_MAGIC.as_bytes(), db_path).unwrap();
    let secp = Secp256k1::new();

    // generate external and internal descriptor from mnemonic
    let (external_descriptor, _ext_keymap) =
        match descriptor!(tr((seed.clone(), derivation_path_external.clone())))
            .unwrap()
            .into_wallet_descriptor(&secp, network)
        {
            Ok((extended_descriptor, keymap)) => (extended_descriptor, keymap),
            Err(e) => panic!("Invalid external derivation path: {}", e),
        };
    let (internal_descriptor, _int_keymap) =
        match descriptor!(tr((seed.clone(), derivation_path_internal.clone())))
            .unwrap()
            .into_wallet_descriptor(&secp, network)
        {
            Ok((extended_descriptor, keymap)) => (extended_descriptor, keymap),
            Err(e) => panic!("Invalid internal derivation path: {}", e),
        };

    Wallet::new(external_descriptor, Some(internal_descriptor), db, network).unwrap()
}

pub fn create_address(input: &str) -> Address {
    match Address::from_str(input) {
        Ok(address) => address,
        Err(e) => panic!("Invalid address: {}", e),
    }
}

pub fn create_transaction(
    wallet: &mut Wallet<Store<ChangeSet>>,
    address: Address,
) -> Result<(PartiallySignedTransaction, TransactionDetails), Error> {
    let mut builder = wallet.build_tx();
    builder
        // Spend all outputs in this wallet.
        .drain_wallet()
        // Send the excess (which is all the coins minus the fee) to this address.
        .drain_to(address.script_pubkey())
        .fee_rate(FeeRate::from_sat_per_vb(5.0))
        .enable_rbf();

    builder.finish()
}

pub async fn sync_wallet(wallet: &mut Wallet<Store<'_, ChangeSet>>, client: &impl EsploraAsyncExt) {
    let local_chain = wallet.checkpoints();

    let keychain_spks = wallet.spks_of_all_keychains().into_iter().collect();
    let update = client
        .scan(
            local_chain,
            keychain_spks,
            [],
            [],
            STOP_GAP,
            PARALLEL_REQUESTS,
        )
        .await;
    match update {
        Ok(update) => {
            wallet.apply_update(update).unwrap();
            wallet.commit().unwrap();
        }
        Err(e) => panic!("Error syncing wallet: {}", e),
    }
}

pub fn check_balance(wallet: &Wallet<Store<ChangeSet>>) -> bool {
    // no need to check for lower than 0 since it is unsigned
    let balance = wallet.get_balance();
    !matches!(balance.confirmed, 0)
}
