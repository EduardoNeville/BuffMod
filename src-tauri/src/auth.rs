use std::sync::Mutex;

use crate::secure_db_access::SecureDbError;
use crate::storage::{new_db, StorageError};
use crate::supabase::{Supabase, SupabaseError};
use crate::{AppState, StateWrapper};

use thiserror::Error;
use serde::Serialize;
use tauri_plugin_stronghold::stronghold::{self};
use base64::{engine::general_purpose, Engine as _};

/// Define a custom AuthError enum for improved handling
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Supabase error: {0}")]
    SupabaseError(#[from] SupabaseError),

    #[error("Key encryption error: {0}")]
    SecureDbError(#[from] SecureDbError),

    #[error("Error storing into DB: {0}")]
    StorageError(#[from] StorageError),

    #[error("Secure DB error: {0}")]
    StrongholdError(#[from] stronghold::Error),

    #[error("Invalid user data format")]
    InvalidUserData,

    #[error("Stronghold instance not available")]
    StrongholdUnavailable,

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
    email: String,
    password: String
) -> Result<Vec<String>, AuthError> {
    let supabase = Supabase::new()?;
    
    // 🔐 Authenticate user with Supabase
    let user_data = supabase.sign_in(&email, &password).await?;
    
    // ✅ Extract tokens
    let access_token = user_data["access_token"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;
    
    let refresh_token = user_data["refresh_token"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;

    let user_id = user_data["user"]["id"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;

    println!("user_id: {:?}", user_id);
    
    // Generate DB encryption key
    let enc_key = crate::secure_db_access::EncKey::new(access_token, refresh_token, user_id)?;
    let db_key = enc_key.derive_encryption_key(user_id)?;

    // 🛠️ Store DB encryption key in app state
    let str_db_key = general_purpose::STANDARD.encode(&db_key);
    let mut loc_state = state.lock().unwrap();
    if let Some(ref mut s) = *loc_state {
        s.db_key = Some(str_db_key.to_owned());
    } else {
        *loc_state = Some(AppState { db_key: Some(str_db_key.to_owned()), db_path: None });
    }

    Ok(vec![
        access_token.to_string(),
        refresh_token.to_string(),
        user_id.to_string(),
        str_db_key
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
) -> Result<Vec<String>, AuthError> {
    let supabase = Supabase::new()?;
    let user_data = supabase.initial_sign_up(&email, &password, &org_name, &user_name).await?;

    let access_token = user_data["access_token"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;
    
    let refresh_token = user_data["refresh_token"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;

    let user_id = user_data["user"]["id"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;

    println!("user_id: {:?}", user_id);
    
    // Generate DB encryption key
    let enc_key = crate::secure_db_access::EncKey::new(access_token, refresh_token, user_id)?;
    let db_key = enc_key.derive_encryption_key(user_id)?;

    // 🛠️ Store DB encryption key in app state
    let str_db_key = general_purpose::STANDARD.encode(&db_key);
    let mut loc_state = state.lock().unwrap();
    if let Some(ref mut s) = *loc_state {
        s.db_key = Some(str_db_key.to_owned());
    } else {
        *loc_state = Some(AppState { db_key: Some(str_db_key.to_owned()), db_path: None });
    }

    new_db(state.to_owned(), &app_handle, user_id)?;

    Ok(vec![
        access_token.to_string(),
        refresh_token.to_string(),
        user_id.to_string(),
        str_db_key
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
    user_name: String
) -> Result<Vec<String>, AuthError> {
    let supabase = Supabase::new()?;
    let user_data = supabase.invite_sign_up(&email, &password, &invite_code, &user_name).await?;

    let access_token = user_data["access_token"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;
    
    let refresh_token = user_data["refresh_token"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;

    let user_id = user_data["user"]["id"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;

    println!("user_id: {:?}", user_id);
    
    // Generate DB encryption key
    let enc_key = crate::secure_db_access::EncKey::new(access_token, refresh_token, user_id)?;
    let db_key = enc_key.derive_encryption_key(user_id)?;

    // 🛠️ Store DB encryption key in app state
    let str_db_key = general_purpose::STANDARD.encode(&db_key);
    let mut loc_state = state.lock().unwrap();
    if let Some(ref mut s) = *loc_state {
        s.db_key = Some(str_db_key.to_owned());
    } else {
        *loc_state = Some(AppState { db_key: Some(str_db_key.to_owned()), db_path: None });
    }

    new_db(state.to_owned(), &app_handle, user_id)?;

    Ok(vec![
        access_token.to_string(),
        refresh_token.to_string(),
        user_id.to_string(),
        str_db_key
    ])
}
