use bdk::keys::bip39::{Language, Mnemonic};

const LANG: Language = Language::English; // TODO: hardcoded mnemonic language english for now

pub fn parse_mnemonic(mnemonic: &str) -> Mnemonic {
    match Mnemonic::parse_in(LANG, mnemonic) {
        Ok(m) => m,
        Err(e) => panic!("Invalid mnemonic: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::TypeId;

    fn is_mnemonic<T: ?Sized + 'static>(_s: &T) -> bool {
        TypeId::of::<Mnemonic>() == TypeId::of::<T>()
    }

    #[test]
    fn test_mnemonic() {
        let mnemonic_12 = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon cactus";
        let mnemonic_24 = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art";
        let parsed_mnemonic_12 = parse_mnemonic(mnemonic_12);
        let parsed_mnemonic_24 = parse_mnemonic(mnemonic_24);
        assert!(is_mnemonic(&parsed_mnemonic_12));
        assert!(is_mnemonic(&parsed_mnemonic_24));
    }

    #[test]
    #[should_panic]
    fn test_invalid_mnemonic_12() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon";
        parse_mnemonic(mnemonic);
    }

    #[test]
    #[should_panic]
    fn test_invalid_mnemonic_24() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon";
        parse_mnemonic(mnemonic);
    }
}
