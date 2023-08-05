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
    FeeRate, SignOptions,
};
use bdk_esplora::{esplora_client::AsyncClient, EsploraAsyncExt};
use bdk_file_store::Store;
use std::str::FromStr;
use std::{collections::HashMap, env::temp_dir};

/// The magic string used to identify the database.
const DB_MAGIC: &str = "sweepr";
/// The number of unused addresses to check before stopping.
const STOP_GAP: usize = 5;
/// The number of parallel requests to send to the esplora server.
const PARALLEL_REQUESTS: usize = 5;

/// Derivation paths for different wallets without the last index.
pub const DERIVATION_PATHS: &[&str; 11] = &[
    "m/44'/0'/0'/",
    "m/48'/0'/0'/",
    "m/49'/0'/0'/",
    "m/84'/0'/0'/",
    "m/47'/0'/0'/",
    "m/84'/0'/2147483644'/",
    "m/84'/0'/2147483645'/",
    "m/44'/0'/2147483646'/",
    "m/49'/0'/2147483646'/",
    "m/84'/0'/2147483646'/",
    "m/86'/0'/0'/",
];

/// Derivation paths for different wallets with the last index.
pub fn create_derivation_paths_with_last_index(input: &str) -> (DerivationPath, DerivationPath) {
    let extenal_str = input.to_owned() + "0";
    let internal_str = input.to_owned() + "1";
    (
        create_derivation_path(extenal_str.as_str()),
        create_derivation_path(internal_str.as_str()),
    )
}

/// Creates a derivation path from a string.
pub fn create_derivation_path(input: &str) -> DerivationPath {
    match DerivationPath::from_str(input) {
        Ok(derivation_path) => derivation_path,
        Err(e) => panic!("Invalid derivation path: {}", e),
    }
}

/// Creates a wallet from a mnemonic, a network type, and an internal and external derivation paths.
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
        match descriptor!(wpkh((seed.clone(), derivation_path_external.clone())))
            .unwrap()
            .into_wallet_descriptor(&secp, network)
        {
            Ok((extended_descriptor, keymap)) => (extended_descriptor, keymap),
            Err(e) => panic!("Invalid external derivation path: {}", e),
        };
    let (internal_descriptor, _int_keymap) =
        match descriptor!(wpkh((seed.clone(), derivation_path_internal.clone())))
            .unwrap()
            .into_wallet_descriptor(&secp, network)
        {
            Ok((extended_descriptor, keymap)) => (extended_descriptor, keymap),
            Err(e) => panic!("Invalid internal derivation path: {}", e),
        };

    Wallet::new(external_descriptor, Some(internal_descriptor), db, network).unwrap()
}

/// Creates an address from a string.
pub fn create_address(input: &str) -> Address {
    match Address::from_str(input) {
        Ok(address) => address,
        Err(e) => panic!("Invalid address: {}", e),
    }
}

/// Create a Signed Transaction from a wallet using all available coins to send to a given address.
/// Estimate the fee using the Esplora client.
/// Tries to use fee rate such that it will be included in the next block.
/// By default, the transaction is marked as RBF.
pub async fn create_signed_transaction(
    wallet: &mut Wallet<Store<'_, ChangeSet>>,
    address: Address,
    client: &AsyncClient,
) -> PartiallySignedTransaction {
    let fee_rate = get_fee_estimates(client, None).await;
    let mut tx_builder = wallet.build_tx();
    tx_builder
        // Spend all outputs in this wallet.
        .drain_wallet()
        // Send the excess (which is all the coins minus the fee) to this address.
        .drain_to(address.script_pubkey())
        .fee_rate(FeeRate::from_sat_per_vb(fee_rate))
        .enable_rbf();

    let (mut psbt, _) = match tx_builder.finish() {
        Ok(psbt) => psbt,
        Err(e) => panic!("Error creating transaction: {}", e),
    };
    match wallet.sign(&mut psbt, SignOptions::default()) {
        Ok(finalized) => finalized,
        Err(e) => panic!("Error signing transaction: {}", e),
    };
    psbt
}

/// Broadcast a signed transaction to the network using the given Esplora client.
pub async fn broadcast_signed_transaction(psbt: PartiallySignedTransaction, client: &AsyncClient) {
    let tx = psbt.extract_tx();
    match client.broadcast(&tx).await {
        Ok(_) => println!("Transaction sent!"),
        Err(e) => panic!("Error broadcasting transaction: {}", e),
    };
    println!("Tx broadcasted! Txid: {}", tx.txid());
}

/// Sync a wallet with the Esplora client.
pub async fn sync_wallet(wallet: &mut Wallet<Store<'_, ChangeSet>>, client: &AsyncClient) {
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

/// Check if a wallet has any coins to spend.
pub fn check_balance(wallet: &Wallet<Store<ChangeSet>>) -> bool {
    // no need to check for lower than 0 since it is unsigned
    let balance = wallet.get_balance();
    !matches!(balance.confirmed, 0)
}

/// Get the fee estimates from the Esplora server.
/// The default block is 1, which is the next block.
pub async fn get_fee_estimates(client: &AsyncClient, block: Option<u64>) -> f32 {
    let fee_estimates: HashMap<String, f64> = match client.get_fee_estimates().await {
        Ok(future) => future,
        Err(e) => panic!("Error getting fee estimates: {}", e),
    };
    let fee_estimate = match block {
        Some(block) => fee_estimates.get(&block.to_string()).unwrap(),
        None => fee_estimates.get("1").unwrap(),
    };
    *fee_estimate as f32
}
