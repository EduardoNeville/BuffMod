use rusqlite::Connection;
use tauri::api::path::config_dir;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use thiserror::Error; 
use crate::secure_db_access::{EncKey, SecureDbError};

/// Custom error type for SecureStorage
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to open database: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Invalid database path")]
    InvalidDbPath,

    #[error("Encryption error")]
    EncryptionError,

    #[error("Secure DB Error: {0}")]
    SecureDbError(#[from] SecureDbError),
}

/// Struct for handling secure database storage
pub struct SecureStorage {
    conn: Option<Connection>,
}

impl SecureStorage {
    /// Initialize the storage with an optional encryption key
    /// 
    /// # Parameters:
    /// - `config_dir`: The directory where the database will be stored.
    /// - `enc_key`: An optional encryption key for the database.
    ///
    /// # Returns:
    /// - `Ok(Self)`: If initialization was successful.
    /// - `Err(StorageError)`: If an error occurs.
    pub fn new(enc_key: Option<&EncKey>) -> Result<Self, StorageError> {
        let mut db_path = config_dir().unwrap_or_else(|| {
            let fallback_path = dirs::home_dir().map(|p| p.join(".buffmod"));
            fallback_path.unwrap_or_else(|| PathBuf::from("./.buffmod"))
        });

        // Ensure the config directory exists
        if !db_path.exists() {
            fs::create_dir_all(&db_path).map_err(|_| StorageError::InvalidDbPath)?;
        }

        db_path.push("buffmod.db");

        let conn = if let Some(enc_key) = enc_key {
            let derived_key = enc_key.derive_encryption_key()?;
            Some(Self::open_encrypted_db(&db_path, &derived_key)?)
        } else {
            None
        };

        Ok(SecureStorage { conn })
    }

    /// Opens an SQLite encrypted database
    fn open_encrypted_db(db_path: &PathBuf, encryption_key: &[u8]) -> Result<Connection, StorageError> {
        let path_str = db_path.to_str().ok_or(StorageError::InvalidDbPath)?;

        let conn = Connection::open(path_str)?;
        
        // Convert key to hex since SQLCipher expects a hexadecimal key
        let hex_key = hex::encode(encryption_key);
        
        // Apply encryption key
        conn.execute(&format!("PRAGMA key = '{}'", hex_key), [])
            .map_err(|_| StorageError::EncryptionError)?;

        Ok(conn)
    }

    /// Lock the database by clearing the connection
    pub fn lock_database(&mut self) {
        self.conn = None;
        println!("🔒 Database locked.");
    }
}
