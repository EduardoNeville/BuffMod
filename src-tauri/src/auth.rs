use crate::secure_db_access::{EncKey, SecureDbError};
use crate::storage::{SecureStorage, StorageError};
use crate::supabase::{Supabase, SupabaseError};

use serde::Serialize;
use serde_json::Value;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tokio::sync::OnceCell;

pub static SECURE_STORAGE: OnceCell<Arc<Mutex<Option<SecureStorage>>>> = OnceCell::const_new();

/// Define a custom AuthError enum for improved handling
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Supabase error: {0}")]
    SupabaseError(#[from] SupabaseError),

    #[error("Secure DB error: {0}")]
    SecureDbError(#[from] SecureDbError),

    #[error("Storage error: {0}")]
    StorageError(#[from] StorageError),

    #[error("Invalid user data format")]
    InvalidUserData,
}

// Implement serde::Serialize so AuthError can be passed through Tauri commands
impl Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

/// Initializes secure storage with the given encryption key
/// and prevents multiple initializations.
fn initialize_secure_storage(enc_key: EncKey) -> Result<(), AuthError> {
    let storage = SecureStorage::new(Some(&enc_key))?;
    SECURE_STORAGE
        .set(Arc::new(Mutex::new(Some(storage))))
        .map_err(|_| AuthError::StorageError(StorageError::EncryptionError))?;
    Ok(())
}

/// Extracts tokens from user_data and initializes SecureStorage
fn extract_and_initialize_storage(user_data: Value) -> Result<(), AuthError> {
    let access_token = user_data["access_token"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;

    let refresh_token = user_data["refresh_token"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;

    // Generate encryption key
    let enc_key = EncKey::new(access_token, refresh_token)?;

    // Initialize storage
    initialize_secure_storage(enc_key)?;

    Ok(())
}

/// Command to handle user sign-in
#[tauri::command]
pub async fn sign_in(email: String, password: String) -> Result<(), AuthError> {
    let supabase = Supabase::new()?;

    let user_data = supabase.sign_in(&email, &password).await?;

    tokio::task::spawn_blocking(move || extract_and_initialize_storage(user_data))
        .await
        .map_err(|_| AuthError::StorageError(StorageError::EncryptionError))??;

    Ok(())
}

/// Command to handle initial user sign-up
#[tauri::command]
pub async fn initial_sign_up(
    email: String,
    password: String,
    org_name: String,
    user_name: String,
) -> Result<(), AuthError> {
    let supabase = Supabase::new()?;
    let user_data = supabase
        .initial_sign_up(&email, &password, &org_name, &user_name)
        .await?;

    tokio::task::spawn_blocking(move || extract_and_initialize_storage(user_data))
        .await
        .map_err(|_| AuthError::StorageError(StorageError::EncryptionError))??;

    Ok(())
}

//#[tauri::command]
//async fn invite_user(org_id: String, email: String) -> Result<String, String> {
//    let supabase = Supabase::new();
//    match supabase.create_invite(&org_id, &email).await {
//        Ok(invite_code) => Ok(invite_code),
//        Err(e) => {
//            eprintln!("invite_user failed: {}", e);
//            Err(e) // Send error back to frontend
//        }
//    }
//}
//
//#[tauri::command]
//async fn invite_sign_up(email: String, password: String, invite_code: String, user_name: String) -> Result<(), String> {
//    let supabase = Supabase::new();
//    match supabase.invite_sign_up(&email, &password, &invite_code, &user_name).await {
//        Ok(_) => Ok(()),
//        Err(e) => {
//            eprintln!("invite_sign_up failed: {}", e);
//            Err(e) // Send error back to frontend
//        }
//    }
//}
