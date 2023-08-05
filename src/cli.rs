use bdk_chain::bitcoin::util::bip32::DerivationPath;
use clap::Parser;
use futures::future::join_all;
use rayon::prelude::*;

use crate::{
    bip39::parse_mnemonic,
    network::{create_client, create_network},
    wallet::{
        broadcast_signed_transaction, check_balance, create_address,
        create_derivation_paths_with_last_index, create_signed_transaction, create_wallet,
        sync_wallet, DERIVATION_PATHS,
    },
};

/// Bitcoin address generator
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Seed to sweep funds from
    seed: String,

    /// Address to withdraw to
    address: String,

    /// Network to use
    #[arg(short, long, default_value = "mainnet")]
    network: String,

    /// Esplora server to use
    #[arg(short, long, default_value = "https://mempool.space/api")]
    url: String,
}

/// Parse command line arguments
fn parse() -> Args {
    Args::parse()
}

/// Run the CLI
/// 1. Parse command line arguments
/// 2. Parse mnemonic
/// 3. Create address from string
/// 4. Create network from string
/// 5. Create derivation paths from the specified derivation paths (11 by default)
/// 6. Create wallets from the specified derivation paths
/// 7. Create an Espora client from the specified url
/// 8. Sync wallets
/// 9. Check balance of wallets
/// 10. Create signed transactions for wallets that have balance
/// 11. Broadcast signed transactions to the Espora server
pub async fn run() {
    let args = parse();
    let seed = parse_mnemonic(&args.seed);
    let address = create_address(&args.address);
    let network = create_network(&args.network);
    let derivation_paths: Vec<(DerivationPath, DerivationPath)> = DERIVATION_PATHS
        .par_iter()
        .map(|path| create_derivation_paths_with_last_index(path))
        .collect();
    let mut wallets: Vec<_> = derivation_paths
        .par_iter()
        .map(|(external, internal)| {
            create_wallet(seed.clone(), network, external.clone(), internal.clone())
        })
        .collect();
    let client = create_client(&args.url);

    // parallel async wallet sync
    let tasks = wallets
        .iter_mut()
        .map(|wallet| sync_wallet(wallet, &client))
        .collect::<Vec<_>>();
    join_all(tasks).await;

    // filter wallets that have balance
    let mut wallets_with_balance: Vec<_> = wallets.into_par_iter().filter(check_balance).collect();

    // parallel async transaction creation
    if !wallets_with_balance.is_empty() {
        let pbsts = join_all(
            wallets_with_balance
                .iter_mut()
                .map(|wallet| create_signed_transaction(wallet, address.clone(), &client))
                .collect::<Vec<_>>(),
        )
        .await;
        join_all(
            pbsts
                .into_iter()
                .map(|psbt| broadcast_signed_transaction(psbt, &client))
                .collect::<Vec<_>>(),
        )
        .await;
    }
}
