use aes_gcm::{Aes256Gcm, Key, Nonce}; // AES-GCM 256-bit
use aes_gcm::aead::{Aead, KeyInit};
use rand::RngCore;
use std::fs;
use base64::{encode, decode};

const NONCE_SIZE: usize = 12;
const KEY_FILE: &str = "vault.key";

pub fn generate_key() -> Vec<u8> {
    if let Ok(key_data) = fs::read(KEY_FILE) {
        return key_data;
    }

    let mut key = vec![0u8; 32]; // 256-bit key
    rand::thread_rng().fill_bytes(&mut key);
    fs::write(KEY_FILE, &key).expect("Failed to write key file");
    key
}

pub fn encrypt(plaintext: &str, key: &[u8]) -> String {
    let cipher = Aes256Gcm::new(Key::from_slice(key));

    let mut nonce = [0u8; NONCE_SIZE];
    rand::thread_rng().fill_bytes(&mut nonce);

    let ciphertext = cipher.encrypt(Nonce::from_slice(&nonce), plaintext.as_bytes()).expect("Encryption failed");

    let mut data = nonce.to_vec();
    data.extend(ciphertext);

    encode(&data) // Base64 encoded string
}

pub fn decrypt(ciphertext_b64: &str, key: &[u8]) -> String {
    let data = decode(ciphertext_b64).expect("Base64 decode failed");

    let (nonce, ciphertext) = data.split_at(NONCE_SIZE);
    let cipher = Aes256Gcm::new(Key::from_slice(key));

    let plaintext = cipher.decrypt(Nonce::from_slice(nonce), ciphertext).expect("Decryption failed");

    String::from_utf8(plaintext).expect("Invalid UTF-8")
}