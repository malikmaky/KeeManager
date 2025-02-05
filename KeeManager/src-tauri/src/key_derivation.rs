use crate::encode;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha256;

const DEFAULT_ITERATIONS: u32 = 1000;
const DEFAULT_KEY_LENGTH: usize = 32;

//
// Derive a cryptographic key from a master key and salt using PBKDF2
// then Base64 encoding the derived key
//
pub fn derive_key(master_key: &str, salt: &str) -> Result<String, bool> {
    let decoded_master_key = match encode::decode_base64(master_key) {
        Ok(key) => key,
        Err(_) => {
            eprintln!("Key Derivation: failed to decode master key from Base64");
            return Err(false);
        }
    };

    let decoded_salt = match encode::decode_base64(salt) {
        Ok(salt) => salt,
        Err(_) => {
            eprintln!("Key Derivation: failed to decode salt from Base64");
            return Err(false);
        }
    };

    let mut derived_key = vec![0u8; DEFAULT_KEY_LENGTH];

    // Derive a key using master key and randomly generated salt
    match pbkdf2::<Hmac<Sha256>>(
        &decoded_master_key,
        &decoded_salt,
        DEFAULT_ITERATIONS,
        &mut derived_key,
    ) {
        Ok(_key) => (),
        Err(e) => {
            eprintln!("PBKDF2 key derivation failed: {}", e);
            return Err(false);
        }
    }

    Ok(encode::encode_base64(derived_key))
}
