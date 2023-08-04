use bdk::bitcoin::{BlockHash, Network, PackedLockTime, Transaction, TxOut, Txid};
use bdk::keys::bip39::Mnemonic;
use bdk::wallet::{AddressIndex, ChangeSet, Wallet};
use bdk_chain::bitcoin::util::bip32::DerivationPath;
use bdk_chain::{bitcoin::hashes::Hash, BlockId, ConfirmationTime};
use bdk_file_store::Store;
use sweepr::wallet::create_wallet;

/// Return a fake wallet that appears to be funded for testing.
pub fn get_funded_wallet_with_change<'a>(
    mnemonic: Mnemonic,
    derivation_path_external: DerivationPath,
    derivation_path_internal: DerivationPath,
) -> (Wallet<Store<'a, ChangeSet>>, Txid) {
    let mut wallet = create_wallet(
        mnemonic,
        Network::Regtest,
        derivation_path_external,
        derivation_path_internal,
    );

    let address = wallet.get_address(AddressIndex::New).address;

    let tx = Transaction {
        version: 1,
        lock_time: PackedLockTime(0),
        input: vec![],
        output: vec![TxOut {
            value: 50_000,
            script_pubkey: address.script_pubkey(),
        }],
    };

    wallet
        .insert_checkpoint(BlockId {
            height: 1_000,
            hash: BlockHash::all_zeros(),
        })
        .unwrap();
    wallet
        .insert_tx(
            tx.clone(),
            ConfirmationTime::Confirmed {
                height: 1_000,
                time: 100,
            },
        )
        .unwrap();

    (wallet, tx.txid())
}
