use rusqlite::Connection;
use serde::Serialize;
use tauri::{AppHandle, Manager};
use std::{fs, path::PathBuf, str::FromStr};
use thiserror::Error; 

use crate::{db_api::{open_encrypted_db, DbApiError}, AppState, StateWrapper};

const CREATE_SCRIPT: &str = include_str!("../migrations/create_tables.sql");

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
    
    #[error("[storage.rs::db_api] SQL error: {0}")]
    SqlError(String),
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
    let mut db_path = app_handle
        .path()
        .data_dir()
        .map_err(StorageError::TauriError)?;

    db_path = db_path.join(format!("buffmod/storage/{}.sqlite", user_id));

    println!("data_path: {:?}", db_path);

    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|_| StorageError::DatabaseError(rusqlite::Error::InvalidPath(db_path.clone())))?;
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
    db_conn.execute_batch(CREATE_SCRIPT)
        .map_err(|e| StorageError::SqlError(e.to_string()))?;

    Ok(db_conn)
}


