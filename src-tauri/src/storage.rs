use rusqlite::Connection;
use serde::Serialize;
use std::{fs, path::PathBuf, str::FromStr};
use tauri::{AppHandle, Manager};
use thiserror::Error;

use crate::{
    db_api::{open_encrypted_db, DbApiError},
    AppState, StateWrapper,
};

const CREATE_SCRIPT: &str = include_str!("../migrations/create_tables.sql");

/// Custom error type for SecureStorage
#[derive(Error, Debug)]
pub enum StorageError {
    #[error("[storage.rs::open_database] Failed to open database: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("[storage.rs::get_database_path] Invalid database path: {path:?}")]
    InvalidDbPath { path: Option<PathBuf> },

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

    let dirs = String::from("buffmod/storage/");
    std::fs::create_dir_all(&dirs).map_err(|e| {
        DbApiError::FileError(format!("Failed to create directory {}: {}", dirs, e))
    })?;
    data_path = data_path.join(dirs);
    data_path = data_path.join(format!("{}.db", user_id));

    println!("data_path: {:?}", data_path);

    if !data_path.exists() {
        std::fs::File::create(&data_path).map_err(|e| {
            DbApiError::FileError(format!("Failed to create db {}: {}", data_path.display(), e))
        })?; 
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
pub fn new_db(
    state: tauri::State<StateWrapper>,
) -> Result<(), StorageError> {
    let state_guard = state.lock().unwrap();
    let db_conn = state_guard.as_ref().and_then(|s| s.db_conn.as_ref()).unwrap();

    // Initialize tables
    db_conn
        .execute_batch(CREATE_SCRIPT)
        .map_err(|e| StorageError::SqlError(e.to_string()))?;
    Ok(())

}
