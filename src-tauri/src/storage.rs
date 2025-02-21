use rusqlite::Connection;
use tauri::api::path::config_dir;
use std::fs;
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
    pub conn: Option<Connection>,
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
            let db_conn = Self::open_encrypted_db(&db_path, &derived_key)?;
            Self::initialize_tables(&db_conn)?;
            Some(db_conn)
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

    fn initialize_tables(conn: &Connection) -> Result<(), StorageError> {
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS clients (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                email TEXT UNIQUE NOT NULL,
                phone TEXT,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                start_date TIMESTAMP NOT NULL,
                end_date TIMESTAMP NOT NULL,
                client_id INTEGER,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE SET NULL
            );

            CREATE TABLE IF NOT EXISTS invoices (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                client_id INTEGER,
                amount REAL NOT NULL,
                due_date TIMESTAMP NOT NULL,
                status TEXT CHECK (status IN ('Paid', 'Pending', 'Overdue')) NOT NULL DEFAULT 'Pending',
                event_id INTEGER,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE SET NULL,
                FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE SET NULL
            );

            CREATE TABLE IF NOT EXISTS expenses (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                amount REAL NOT NULL,
                category TEXT NOT NULL,
                date TIMESTAMP NOT NULL,
                event_id INTEGER,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE SET NULL
            );

            CREATE TABLE IF NOT EXISTS social_media_posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                platform TEXT NOT NULL,
                content TEXT NOT NULL,
                schedule_time TIMESTAMP NOT NULL,
                event_id INTEGER,
                client_id INTEGER,
                status TEXT CHECK (status IN ('Scheduled', 'Posted')) NOT NULL DEFAULT 'Scheduled',
                FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE SET NULL,
                FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE SET NULL
            );

            CREATE TABLE IF NOT EXISTS permissions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                role TEXT CHECK (role IN ('admin', 'editor', 'viewer')) NOT NULL
            );

            CREATE TABLE IF NOT EXISTS settings (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT UNIQUE NOT NULL,
                value TEXT
            );
            "
        )?;
        println!("✅ Tables initialized successfully.");
        Ok(())
    }

    /// Lock the database by clearing the connection
    pub fn lock_database(&mut self) {
        self.conn = None;
        println!("🔒 Database locked.");
    }
}
