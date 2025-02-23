use rusqlite::Connection;
use tauri::path::PathResolver;
use tauri::{AppHandle, Manager};
use std::fs;
use std::path::PathBuf;
use thiserror::Error; 

use crate::AppState;

/// Custom error type for SecureStorage
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Failed to open database: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Invalid database path")]
    InvalidDbPath,

    #[error("Encryption error")]
    EncryptionError,

}

fn get_database_path(app_handle: &AppHandle) -> Result<PathBuf, StorageError> {
    let data_dir = app_handle.path().data_dir().map_err(|_| StorageError::InvalidDbPath)?;
    let db_path = data_dir.join("buffmod.sqlite");

    // Ensure the directory exists
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir).map_err(|_| StorageError::InvalidDbPath)?;
    }

    Ok(db_path)
}

/// Initialize the storage with an optional encryption key
/// 
/// # Parameters:
/// - `state`: State of the App
/// - `app_handle` : Handler (with path)
///
/// # Returns:
/// - `Ok(Connection)`: If initialization was successful.
/// - `Err(StorageError)`: If an error occurs.
/// Initialize the storage with encryption
pub fn new_db(state: tauri::State<AppState>, app_handle: &AppHandle) -> Result<Connection, StorageError> {

    let db_key = state.db_key.lock().unwrap().clone().unwrap();
    let db_conn = open_encrypted_db(&app_handle, &db_key)?;
    
    initialize_tables(&db_conn)?;

    Ok(db_conn)
}

/// Opens an SQLite encrypted database
pub fn open_encrypted_db(app_handle: &AppHandle, encryption_key: &str) -> Result<Connection, StorageError> {

    let db_path = get_database_path(app_handle)?;
    let conn = Connection::open(db_path)?;
    
    conn.execute(&format!("PRAGMA key = '{}'", encryption_key), [])
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

