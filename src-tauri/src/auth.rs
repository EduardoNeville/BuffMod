use crate::supabase::{Supabase, SupabaseError};
use crate::AppState;

use thiserror::Error;
use serde::Serialize;
use tauri_plugin_stronghold::stronghold;

/// Define a custom AuthError enum for improved handling
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Supabase error: {0}")]
    SupabaseError(#[from] SupabaseError),

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

/// Command to handle user sign-in
#[tauri::command]
pub async fn sign_in(state: tauri::State<'_, AppState>, email: String, password: String) -> Result<(), AuthError> {
    let supabase = Supabase::new()?;

    let user_data = supabase.sign_in(&email, &password).await?;

    // Storing tokens
    let access_token = user_data["access_token"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;
    
    let refresh_token = user_data["refresh_token"]
        .as_str()
        .ok_or(AuthError::InvalidUserData)?;

    let mut stronghold_guard = state.stronghold.lock().unwrap();
    let sth = stronghold_guard.as_mut().ok_or(AuthError::StrongholdUnavailable)?;
    let store = sth.store();

    store.insert("access_token".as_bytes().to_vec(), access_token.as_bytes().to_vec(), None).expect("Failed to store Access Token");
    store.insert("refresh_token".as_bytes().to_vec(), refresh_token.as_bytes().to_vec(), None).expect("Failed to store Refresh Token");

    sth.save();

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
    let user_data = supabase.initial_sign_up(&email, &password, &org_name, &user_name).await?;

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
