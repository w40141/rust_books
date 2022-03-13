extern crate block_modes;

use aes::cipher::block_padding::Pkcs7;
use aes::Aes256;
use getrandom::getrandom;
use sha2::{Digest, Sha256};

type AesCbc = Cbc<Aes256, Pkcs7>;
const SALT: &str = "LFsMH#kL!IfY:dcEz9F/dvj17nUN";

pub fn encrypt(password: &str, data: &str) -> String {
    let key = get_key(password);
    let iv = gen_iv();
    let cipher = AesCbc::new_from_slices(&key, &iv);
    let encryption = cipher.encrypt_vec(data.as_bytes());
    let mut result: Vec<u8> = vec![];
    result.extend(iv);
    result.extend(encryption);
    base64::encode(result)
}

fn gen_iv() -> Vec<u8> {
    let mut res: Vec<u8> = vec![0; 16];
    getrandom(&mut res).unwrap();
    res
}

fn get_key(password: &str) -> Vec<u8> {
    let pw: String = format!("{password}::{SALT}");
    let mut h = Sha256::new();
    h.update(pw.as_bytes());
    h.finalize().to_vec()
}

pub fn decrypt(password: &str, data: &str) -> String {
    let key = get_key(password);
    let bytes = base64::decode(data).unwrap();
    let iv = &bytes[..16];
    let cipher = AesCbc::new_from_slices(&key, iv);
    let result = cipher.decrypt_vec(&bytes[16..]);
    String::from_utf8(result).unwrap()
}

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
