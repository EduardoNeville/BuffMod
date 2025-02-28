use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::Utc;
use pbkdf2::pbkdf2_hmac;
use rand::RngCore;
use serde_json::Value;
use sha2::digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use thiserror::Error;
use tokio::time::sleep;

const ITERATIONS: u32 = 100_000;
static SESSION_EXPIRY_BUFFER: i64 = 10; // Refresh buffer in seconds

/// Custom error type for Secure Database Access
#[derive(Error, Debug)]
pub enum SecureDbError {
    #[error("Invalid JWT format")]
    InvalidJwtFormat,

    #[error("Failed to decode Base64")]
    Base64DecodeError(#[from] base64::DecodeError),

    #[error("Failed to parse session ID")]
    SessionIdParseError,

    #[error("Token format error")]
    TokenFormatError,

    #[error("Salt generation failed")]
    SaltGenerationError,

    #[error("UTF-8 Conversion error")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("JSON parsing error")]
    JsonParseError(#[from] serde_json::Error),

    #[error("Session has expired")]
    SessionExpiredError,
}

/// Represents the encryption key used for database encryption.
pub struct EncKey {
    access_token: String,
    refresh_token: String,
    pub salt: Vec<u8>,
}

impl EncKey {
    pub fn new(access_token: &str, refresh_token: &str, user_id: &str) -> Result<Self, SecureDbError> {
        let salt = Self::get_persistent_salt(user_id)?;  // Use the same salt for this user
        Ok(EncKey {
            access_token: access_token.to_owned(),
            refresh_token: refresh_token.to_owned(),
            salt,
        })
    }

    /// ✅ Derives an AES encryption key using `user_id + refresh_token`
    pub fn derive_encryption_key(&self, user_id: &str) -> Result<Vec<u8>, SecureDbError> {
        let mut key = [0u8; 32];

        let derived_input = format!("{}{}", user_id, &self.refresh_token); // Stable input!
        pbkdf2_hmac::<Sha256>(derived_input.as_bytes(), &self.salt, ITERATIONS, &mut key);

        Ok(key.to_vec()) // Always returns the same key for this user
    }

    /// 🔥 Get a Persistent Salt Based on User ID  
    fn get_persistent_salt(user_id: &str) -> Result<Vec<u8>, SecureDbError> {
        let mut salt = [0u8; 16]; 

        let user_hash: GenericArray<u8, U32> = Sha256::digest(user_id.as_bytes());
        salt.copy_from_slice(&user_hash[..16]); // Use part of the hash as salt  

        Ok(salt.to_vec()) // Persistent!
    }

    /// Generate a new random salt
    fn generate_salt() -> Result<Vec<u8>, SecureDbError> {
        let mut salt = [0u8; 16]; // 128-bit salt
        rand::rng().fill_bytes(&mut salt);
        Ok(salt.to_vec())
    }

    /// Extract the session_id from a JWT token
    fn get_session_id(&self) -> Result<String, SecureDbError> {
        let token_parts: Vec<&str> = self.access_token.split('.').collect();
        if token_parts.len() != 3 {
            return Err(SecureDbError::InvalidJwtFormat);
        }

        // Ensure base64-url decoding with proper error handling
        let payload = URL_SAFE_NO_PAD.decode(token_parts[1])?;

        let payload_json: Value = serde_json::from_slice(&payload)?;

        payload_json["session_id"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or(SecureDbError::SessionIdParseError)
    }

    /// Monitors token expiry and auto-logs out when the session expires
    async fn monitor_token_expiry(
        &self,
        enc_key: Arc<Mutex<Option<Vec<u8>>>>,
    ) -> Result<(), SecureDbError> {
        if let Some(expiry) = &self.extract_token_expiry()? {
            let now = Utc::now().timestamp();
            let time_left = expiry - now - SESSION_EXPIRY_BUFFER;

            if time_left > 0 {
                sleep(Duration::from_secs(time_left as u64)).await;
                Self::auto_logout(enc_key).await;
            } else {
                println!("⚠️ Token already expired. Logging out...");
                Self::auto_logout(enc_key).await;
            }
        }

        Ok(())
    }

    /// Extracts expiration timestamp from the JWT
    fn extract_token_expiry(&self) -> Result<Option<i64>, SecureDbError> {
        let token_parts: Vec<&str> = self.access_token.split('.').collect();
        if token_parts.len() != 3 {
            return Err(SecureDbError::InvalidJwtFormat);
        }

        let payload = URL_SAFE_NO_PAD.decode(token_parts[1])?;
        let payload_json: Value = serde_json::from_slice(&payload)?;

        Ok(payload_json["exp"].as_i64())
    }

    /// Auto logout and memory cleanup
    async fn auto_logout(enc_key: Arc<Mutex<Option<Vec<u8>>>>) {
        println!("🔒 Session expired! Logging out...");

        let mut key = enc_key.lock().unwrap();
        *key = None;

        println!("🛑 Database access revoked. Please log in again.");
    }
}
