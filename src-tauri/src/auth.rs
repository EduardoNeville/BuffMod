use crate::secure_db_access::SecureDbError;
use crate::supabase::{Supabase, SupabaseError};
use crate::AppState;

use thiserror::Error;
use serde::Serialize;
use tauri_plugin_stronghold::stronghold::{self, Stronghold};

/// Define a custom AuthError enum for improved handling
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Supabase error: {0}")]
    SupabaseError(#[from] SupabaseError),

    #[error("Key encryption error: {0}")]
    SecureDbError(#[from] SecureDbError),

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

fn initialize_stronghold(state: &tauri::State<AppState>, db_key: &[u8]) -> Result<(), AuthError> {
    let mut stronghold_guard = state.stronghold.lock().unwrap();

    // ✅ Initialize only if it hasn't been initialized yet
    if stronghold_guard.is_none() {
        let salt_path = std::env::temp_dir().join("salt.txt"); // Adjust as needed
        //let stronghold_instance = tauri_plugin_stronghold::Builder::with_argon2(&salt_path);
        let stronghold_instance = Stronghold::new(
            salt_path,
            db_key.to_vec()
        )?;
        
        *stronghold_guard = Some(stronghold_instance);
        println!("🔐 Stronghold initialized successfully!");
    } else {
        println!("⚡ Stronghold instance already initialized.");
    }

    Ok(())
}

#[tauri::command]
pub async fn sign_in(state: tauri::State<'_, AppState>, email: String, password: String) -> Result<(), AuthError> {

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

    
    // Generate DB encryption key
    let enc_key = crate::secure_db_access::EncKey::new(access_token, refresh_token)?;
    let db_key = enc_key.derive_encryption_key()?;

    // Ensure Stronghold is initialized before use
    initialize_stronghold(&state, &db_key)?;

    // Obtain stronghold instance (now guaranteed to exist)
    let stronghold_guard = state.stronghold.lock().unwrap();
    let stronghold_instance = stronghold_guard.as_ref().ok_or(AuthError::StrongholdUnavailable)?;
    let store = stronghold_instance.store();

    println!("Store created:");

    // 💾 Store tokens in Stronghold
    store.insert("access_token".as_bytes().to_vec(), access_token.as_bytes().to_vec(), None).expect("Failed to store Access Token");

    println!("Inserted access_token");
    store.insert("refresh_token".as_bytes().to_vec(), refresh_token.as_bytes().to_vec(), None).expect("Failed to store Refresh Token");
    println!("Inserted refresh_token");

    // ✅ Save Stronghold state
    stronghold_instance.save()?;
    println!("✅ Tokens securely stored in Stronghold.");

    // 🛠️ Store DB encryption key in app state
    *state.db_key.lock().unwrap() = Some(base64::encode(db_key));
    println!("✅ Database encryption key derived and stored securely.");

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
