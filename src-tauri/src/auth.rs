use crate::db_api::{open_encrypted_db, DbApiError};
use crate::secure_db_access::SecureDbError;
use crate::storage::{get_database_path, new_db, StorageError};
use crate::supabase::{Supabase, SupabaseError};
use crate::{AppState, StateWrapper};

use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;
use tauri_plugin_stronghold::stronghold::{self};
use thiserror::Error;

/// Define a custom AuthError enum for improved handling
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("[auth.rs::supabase_error] Supabase API Error: {0}")]
    SupabaseError(#[from] SupabaseError),

    #[error("[auth.rs::secure_db_access] Database encryption error: {0}")]
    SecureDbError(#[from] SecureDbError),

    #[error("[auth.rs::storage_error] Storage handling error: {0}")]
    StorageError(#[from] StorageError),

    #[error("[auth.rs::db_api] Storage handling error: {0}")]
    DbApiError(#[from] DbApiError),

    #[error("[auth.rs::stronghold_error] Secure Stronghold error: {0}")]
    StrongholdError(#[from] stronghold::Error),

    #[error("[auth.rs::invalid_user_data] User data format is invalid.")]
    InvalidUserData,

    #[error("[auth.rs::stronghold_unavailable] The Stronghold instance is not available.")]
    StrongholdUnavailable,
}

#[derive(Serialize)]
pub struct Entry {
    pub key: String,
    pub value: String,
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

#[tauri::command]
pub async fn sign_in(
    state: tauri::State<'_, StateWrapper>,
    app_handle: tauri::AppHandle,
    email: String,
    password: String,
) -> Result<Vec<Entry>, AuthError> {
    let supabase = Supabase::new().map_err(|e| AuthError::SupabaseError(e))?;

    let user_id = supabase
        .sign_in(&email, &password)
        .await
        .map_err(|e| AuthError::SupabaseError(e))?;

    let enc_key =
        crate::secure_db_access::EncKey::new(&user_id).map_err(|e| AuthError::SecureDbError(e))?;

    let db_key = enc_key
        .derive_encryption_key(&user_id)
        .map_err(|e| AuthError::SecureDbError(e))?;

    // DB Connection
    let str_db_key = general_purpose::STANDARD.encode(&db_key);
    let db_path = get_database_path(&app_handle, &&user_id)?;
    let db_conn = open_encrypted_db(&db_path, &str_db_key)?;
    {
        let mut loc_state = state.lock().unwrap();

        // ✅ Update AppState with the new database path
        if let Some(ref mut s) = *loc_state {
            s.db_conn = Some(db_conn);
        } else {
            *loc_state = Some(AppState {
                db_conn: Some(db_conn),
            });
        }
    }

    new_db(state.to_owned())?;


    Ok(vec![
        Entry {
            key: "db_key".to_string(),
            value: str_db_key.to_string(),
        },
        Entry {
            key: "user_id".to_string(),
            value: user_id.to_string(),
        },
    ])
}

/// Command to handle initial user sign-up
#[tauri::command]
pub async fn initial_sign_up(
    state: tauri::State<'_, StateWrapper>,
    app_handle: tauri::AppHandle,
    email: String,
    password: String,
    org_name: String,
    user_name: String,
) -> Result<Vec<Entry>, AuthError> {
    let supabase = Supabase::new()?;
    let user_id = supabase
        .initial_sign_up(&email, &password, &org_name, &user_name)
        .await?;


    // Generate DB encryption key
    let enc_key = crate::secure_db_access::EncKey::new(&user_id)?;
    let db_key = enc_key.derive_encryption_key(&user_id)?;

    // DB Connection
    let str_db_key = general_purpose::STANDARD.encode(&db_key);
    let db_path = get_database_path(&app_handle, &&user_id)?;
    let db_conn = open_encrypted_db(&db_path, &str_db_key)?;
    {
        let mut loc_state = state.lock().unwrap();

        // ✅ Update AppState with the new database path
        if let Some(ref mut s) = *loc_state {
            s.db_conn = Some(db_conn);
        } else {
            *loc_state = Some(AppState {
                db_conn: Some(db_conn),
            });
        }
    }

    new_db(state.to_owned())?;

    Ok(vec![
        Entry {
            key: "db_key".to_string(),
            value: str_db_key.to_string(),
        },
        Entry {
            key: "user_id".to_string(),
            value: user_id.to_string(),
        },
    ])
}

#[tauri::command]
async fn invite_user(org_id: String, email: String) -> Result<String, AuthError> {
    let supabase = Supabase::new()?;
    let invite_code = supabase.create_an_invite(&org_id, &email).await?;
    Ok(invite_code)
}

#[tauri::command]
async fn invite_sign_up(
    state: tauri::State<'_, StateWrapper>,
    app_handle: tauri::AppHandle,
    email: String,
    password: String,
    invite_code: String,
    user_name: String,
) -> Result<Vec<String>, AuthError> {
    let supabase = Supabase::new()?;
    let user_data = supabase
        .invite_sign_up(&email, &password, &invite_code, &user_name)
        .await?;

    let user_id = user_data["user"]["id"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;

    // Generate DB encryption key
    let enc_key = crate::secure_db_access::EncKey::new(user_id)?;
    let db_key = enc_key.derive_encryption_key(user_id)?;

    // DB Connection
    let str_db_key = general_purpose::STANDARD.encode(&db_key);
    let db_path = get_database_path(&app_handle, &&user_id)?;
    let db_conn = open_encrypted_db(&db_path, &str_db_key)?;
    {
        let mut loc_state = state.lock().unwrap();

        // ✅ Update AppState with the new database path
        if let Some(ref mut s) = *loc_state {
            s.db_conn = Some(db_conn);
        } else {
            *loc_state = Some(AppState {
                db_conn: Some(db_conn),
            });
        }
    }

    new_db(state.to_owned())?;

    Ok(vec![user_id.to_string(), str_db_key])
}
