use aes::Aes256;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use sha2::{Digest, Sha256};

type AesCbc = Cbc<Aes256, Pkcs7>;
const SALT: &str = "LFsMH#kL!IfY:dcEz9F/dvj17nUN";

pub fn encrypt(password: &str, data: &str) -> String {}

fn gen_iv() -> Vec<u8> {}

fn get_key(password: &str) -> Vec<u8> {}

pub fn decrypt(password: &str, data: &str) -> String {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enc_dec_test() {
        let password = "abcd";
        let data = "I am a hero.";
        let enc = encrypt(password, data);
        println!("encrypt: {enc}");
        let dec = decrypt(password, &enc);
        println!("decrypt: {dec}");
        assert_eq!(data, dec);
    }
}
