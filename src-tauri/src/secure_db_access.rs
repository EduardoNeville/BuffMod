use pbkdf2::pbkdf2_hmac;
use sha2::digest::consts::U32;
use sha2::digest::generic_array::GenericArray;
use sha2::{Digest, Sha256};
use thiserror::Error;

const ITERATIONS: u32 = 100_000;
static SESSION_EXPIRY_BUFFER: i64 = 10; // Refresh buffer in seconds

/// Custom error type for Secure Database Access
#[derive(Error, Debug)]
pub enum SecureDbError {
    #[error("UTF-8 Conversion error")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("JSON parsing error")]
    JsonParseError(#[from] serde_json::Error),

}

/// Represents the encryption key used for database encryption.
pub struct EncKey {
    pub salt: Vec<u8>,
}

impl EncKey {
    pub fn new(user_id: &str) -> Result<Self, SecureDbError> {
        let salt = Self::get_persistent_salt(user_id)?;  // Use the same salt for this user
        Ok(EncKey {
            salt,
        })
    }

    /// Derives an AES encryption key using user_id
    pub fn derive_encryption_key(&self, user_id: &str) -> Result<Vec<u8>, SecureDbError> {
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha256>(user_id.as_bytes(), &self.salt, ITERATIONS, &mut key);
        Ok(key.to_vec()) // Always returns the same key for this user
    }

    /// 🔥 Get a Persistent Salt Based on User ID  
    fn get_persistent_salt(user_id: &str) -> Result<Vec<u8>, SecureDbError> {
        let mut salt = [0u8; 16]; 
        let user_hash: GenericArray<u8, U32> = Sha256::digest(user_id.as_bytes());
        salt.copy_from_slice(&user_hash[..16]); // Use part of the hash as salt  
        Ok(salt.to_vec()) // Persistent!
    }
}
