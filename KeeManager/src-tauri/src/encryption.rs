use crate::{encode, hash};
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::Aes256Gcm;
use rand::RngCore;
use std::error::Error;
use std::fs;

const DEFAULT_SALT_LENGTH: usize = 16;
const DEFAULT_IV_LENGTH: usize = 12;

//
// Generate a random 16 byte salt
//
pub fn generate_salt() -> String {
    let mut salt = [0u8; DEFAULT_SALT_LENGTH];
    OsRng.fill_bytes(&mut salt);
    encode::encode_base64(salt.to_vec())
}

pub fn encrypt_gcm(plaintext: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let decoded_key = match encode::decode_base64(key) {
        Ok(decoded_key) => decoded_key,
        Err(e) => {
            eprintln!("Encryption: key decoding failure: {}", e);
            return Err(e);
        }
    };

    // Initialize the AES-GCM cipher
    let key = GenericArray::from_slice(&decoded_key);
    let cipher = Aes256Gcm::new(key);

    // Generate a random 12-byte IV (nonce)
    let mut iv = [0u8; DEFAULT_IV_LENGTH];
    aes_gcm::aead::OsRng.fill_bytes(&mut iv);
    let iv_slice = GenericArray::from_slice(&iv);

    let ciphertext = match cipher.encrypt(iv_slice, plaintext.as_bytes()) {
        Ok(ciphertext) => ciphertext,
        Err(e) => {
            eprintln!("Encryption: text encrypting failure: {}", e);
            return Err("Text encrypting failure".into());
        }
    };

    // Combine the ciphertext and IV
    let mut result = ciphertext;
    result.extend_from_slice(&iv);

    Ok(encode::encode_base64(result))
}

pub fn decrypt_gcm(encoded_data: &str, key: &str) -> Result<String, Box<dyn Error>> {
    let decoded_data = match encode::decode_base64(encoded_data) {
        Ok(decoded_data) => decoded_data,
        Err(e) => {
            eprintln!("Decryption: cipher text decoding failure: {}", e);
            return Err(e);
        }
    };

    let decoded_key = match encode::decode_base64(key) {
        Ok(decoded_key) => decoded_key,
        Err(e) => {
            eprintln!("Decryption: key decoding failure: {}", e);
            return Err(e);
        }
    };

    let key = GenericArray::from_slice(&decoded_key);
    let cipher = Aes256Gcm::new(key);
        
    // Split the decoded data into the ciphertext and the IV
    let (ciphertext, iv) = decoded_data.split_at(decoded_data.len() - DEFAULT_IV_LENGTH);

    let iv = GenericArray::from_slice(iv);

    let plaintext = match cipher.decrypt(iv, ciphertext) {
        Ok(plaintext) => plaintext,
        Err(e) => {
            eprintln!("Decryption: cipher text decryption failure: {}", e);
            return Err("Cipher text decryption failure".into());
        }
    };
    let plaintext_str = match String::from_utf8(plaintext) {
        Ok(plaintext_str) => plaintext_str,
        Err(e) => {
            eprintln!(
                "Decryption: cipher text to Unicode conversion failure: {}",
                e
            );
            return Err("Cipher text to Unicode conversion failure".into());
        }
    };

    Ok(plaintext_str.to_string())
}

pub fn encrypt_database(db_path: &str, master_key: &str) -> bool {
    let key_bytes = match hash::hash_sha256_as_bytes(master_key.as_bytes().to_vec()) {
        Ok(key_bytes) => key_bytes,
        Err(_) => {
            eprintln!("DB_Encryption: master key hashing failure");
            return false;
        },
    };

    let data = match fs::read(db_path) {
        Ok(data) => data,
        Err(_) => {
            eprintln!("DB_Encryption: DB file reading failure");
            return false;
        },
    };

    // Generate a random nonce
    let mut nonce = [0u8; DEFAULT_IV_LENGTH];
    OsRng.fill_bytes(&mut nonce);

    // Initialize the AES-GCM cipher
    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key_bytes));

    let ciphertext = match cipher.encrypt(GenericArray::from_slice(&nonce), data.as_ref()) {
        Ok(ciphertext) => ciphertext,
        Err(_) => return false,
    };

    let mut encrypted_data = Vec::new();
    encrypted_data.extend_from_slice(&nonce);
    encrypted_data.extend_from_slice(&ciphertext);

    if fs::write(db_path, encrypted_data).is_err() {
        eprintln!("DB_Encryption: DB file writing failure");
        return false;
    }

    true
}

pub fn decrypt_database(db_path: &str, master_key: &str) -> bool {
    let key_bytes = match hash::hash_sha256_as_bytes(master_key.as_bytes().to_vec()) {
        Ok(key_bytes) => key_bytes,
        Err(_) => {
            eprintln!("DB_Decryption: master key hashing failure");
            return false;
        },
    };

    let encrypted_data = match fs::read(db_path) {
        Ok(encrypted_data) => encrypted_data,
        Err(_) => {
            eprintln!("DB_Decryption: DB file reading failure");
            return false;
        },
    };

    // Extract the nonce (first 12 bytes) and the ciphertext
    let (nonce, ciphertext) = encrypted_data.split_at(DEFAULT_IV_LENGTH);

    let cipher = Aes256Gcm::new(GenericArray::from_slice(&key_bytes));

    let decrypted_data = match cipher.decrypt(GenericArray::from_slice(nonce), ciphertext) {
        Ok(decrypted_data) => decrypted_data,
        Err(e) => {
            eprintln!("DB_Decryption: decryption failed: {}", e);
            return false; 
        }
    };

    if fs::write(db_path, decrypted_data).is_err() {
        eprintln!("DB_Decryption: DB file writing failure");
        return false; 
    }

    true
}