use crate::encode;
use sha2::{Digest, Sha256};

//
// Hash given input bytes using SHA-256 and encode to Base64
//
pub fn hash_sha256(input: Vec<u8>) -> Result<String, String> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    let encoded_hash = encode::encode_base64(result.as_slice().to_vec());
    Ok(encoded_hash)
}

//
// Hash given input bytes using SHA-256 ( returns hash as bytes )
//
pub fn hash_sha256_as_bytes(input: Vec<u8>) -> Result<[u8; 32], String> {
    let mut hasher = Sha256::new();
    hasher.update(input);
    let result = hasher.finalize();

    let mut key = [0u8; 32];
    key.copy_from_slice(&result[..32]);
    Ok(key)
}
