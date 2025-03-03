use rusqlite::Connection;
use serde::Serialize;
use tauri::{AppHandle, Manager};
use std::path::PathBuf;
use thiserror::Error; 

use crate::{db_api::{open_encrypted_db, DbApiError}, AppState, StateWrapper};

/// Custom error type for SecureStorage
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("[storage.rs::open_database] Failed to open database: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("[storage.rs::get_database_path] Invalid database path: {path:?}")]
    InvalidDbPath {
        path: Option<PathBuf>
    },

    #[error("[storage.rs::tauri_error] Tauri error encountered: {0}")]
    TauriError(#[from] tauri::Error),

    #[error("[storage.rs::db_api] Database API error: {0}")]
    DbApiError(#[from] DbApiError),
}

impl Serialize for StorageError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

pub fn get_database_path(app_handle: &AppHandle, user_id: &str) -> Result<PathBuf, StorageError> {
    let mut data_path = app_handle
        .path()
        .data_dir()
        .map_err(StorageError::TauriError)?;

    data_path = data_path.join(format!("buffmod/storage/{}.sqlite", user_id));

    println!("data_path: {:?}", data_path);

    if !data_path.exists() {
        return Err(StorageError::InvalidDbPath { path: Some(data_path.clone()) });
    }

    Ok(data_path)
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
pub fn new_db(state: tauri::State<StateWrapper>, app_handle: &AppHandle, user_id: &str) -> Result<Connection, StorageError> {
    let mut loc_state = state.lock().unwrap(); 
    let db_key = loc_state.as_ref().and_then(|s| s.db_key.clone());

    // Get DB path, or create the missing directory
    let db_path = match get_database_path(app_handle, user_id) {
        Ok(path) => path,
        Err(StorageError::InvalidDbPath { path: Some(missing_path) }) => {
            // Ensure parent directory exists
            if let Some(parent) = missing_path.parent() {
                std::fs::create_dir_all(parent)
                    .map_err(|_| StorageError::DatabaseError(rusqlite::Error::InvalidPath(missing_path.clone())))?;
            }
            missing_path
        },
        Err(e) => return Err(e),  // Forward other unexpected errors
    };

    // ✅ Update AppState with the new database path
    if let Some(ref mut s) = *loc_state {
        s.db_path = Some(db_path.clone());
    } else {
        *loc_state = Some(AppState { db_key: db_key.to_owned(), db_path: Some(db_path.clone()) });
    }

    // Open the database with encryption
    let db_conn = open_encrypted_db(&db_path, &db_key.unwrap())?;

    // Initialize tables
    initialize_tables(&db_conn)?;

    Ok(db_conn)
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

