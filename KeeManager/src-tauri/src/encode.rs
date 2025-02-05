use base64::{engine::general_purpose, Engine};

//
// Encode given input bytes to Base64
//
pub fn encode_base64(input: Vec<u8>) -> String {
    general_purpose::STANDARD.encode(input)
}

//
// Decode given input string to bytes
//
pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let decoded = match general_purpose::STANDARD.decode(encoded) {
        Ok(decoded) => decoded,
        Err(e) => {
            eprintln!("Failed to decode input str: {}", e);
            return Err(Box::new(e));
        }
    };

    Ok(decoded)
}
