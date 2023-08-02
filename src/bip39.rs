use bdk::keys::bip39::{Language, Mnemonic};

const LANG: Language = Language::English; // TODO: hardcoded mnemonic language english for now

pub fn parse_mnemonic(mnemonic: &str) -> Mnemonic {
    match Mnemonic::parse_in(LANG, mnemonic) {
        Ok(m) => m,
        Err(e) => panic!("Invalid mnemonic: {}", e),
    }
}
