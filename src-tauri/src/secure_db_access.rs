use base64::{engine::general_purpose::{STANDARD, URL_SAFE}, Engine as _};
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::Value;

const ITERATIONS: u32 = 100_000;

pub struct EncKey {
    access_token: String,
    refresh_token: String
}

impl EncKey {
    pub fn new(access_token: &str, refresh_token: &str) -> Self {
        EncKey {
            access_token: access_token.to_owned(),
            refresh_token: refresh_token.to_owned()
        }
    }

    pub fn generate_enc_key(&self) -> Vec<u8> {
        let session_id = &self.get_session_id().expect("Failed to extract session_id");

        // Generate or retrieve salt
        let salt = &self.generate_salt();
        let salt_encoded = &self.save_salt_base64(&salt);
        
        // Derive encryption key
        self.derive_encryption_key(&session_id, &salt)
    }

    /// Generates a 256-bit encryption key from session_id and refresh_token
    fn derive_encryption_key(&self, session_id: &str, salt: &[u8]) -> Vec<u8> {
        let mut key = [0u8; 32];  // 256-bit key
        let derived_input = format!("{}{}", session_id, &self.refresh_token);
        
        pbkdf2_hmac::<Sha256>(derived_input.as_bytes(), salt, ITERATIONS, &mut key);
        
        key.to_vec()
    }

    /// Generates a new random salt (store locally)
    fn generate_salt(&self) -> Vec<u8> {
        let mut salt = [0u8; 16];  // 128-bit salt
        rand::fill(&mut salt);
        salt.to_vec()
    }

    /// Encodes salt as base64 for storage
    fn save_salt_base64(&self, salt: &[u8]) -> String {
        STANDARD.encode(salt)
    }

    /// Decodes stored base64 salt
    fn load_salt_base64(&self, salt_encoded: &str) -> Vec<u8> {
        STANDARD.decode(salt_encoded).expect("Failed to decode salt")
    }

    /// Extract the session_id from the JWT
    fn get_session_id(&self) -> Option<String> {
        let token_parts: Vec<&str> = self.access_token.split('.').collect();
        if token_parts.len() != 3 {
            return None;  // Invalid JWT format
        }

        let decoded_payload = URL_SAFE.decode(token_parts[1]).ok()?;
        let payload_str = String::from_utf8(decoded_payload).ok()?;
        let payload: Value = serde_json::from_str(&payload_str).ok()?;
        
        payload["session_id"].as_str().map(|s| s.to_string())
    }
}
