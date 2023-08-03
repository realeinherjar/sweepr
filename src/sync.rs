use bdk::blockchain::esplora::EsploraBlockchain;
use bdk::database::MemoryDatabase;
use bdk::wallet::{SyncOptions, Wallet};

pub async fn sync_wallet(wallet: &Wallet<MemoryDatabase>, blockchain: &EsploraBlockchain) {
    let default_sync = SyncOptions { progress: None };
    match wallet.sync(blockchain, default_sync).await {
        Ok(_) => (),
        Err(e) => panic!("Error syncing wallet: {}", e),
    }
}

pub fn check_balance(wallet: &Wallet<MemoryDatabase>) -> bool {
    match wallet.get_balance() {
        Ok(balance) => !matches!(balance.confirmed, 0),
        Err(e) => panic!("Error checking balance: {}", e),
    }
}
