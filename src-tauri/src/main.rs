pub mod supabase;
pub mod secure_db_access;
pub mod storage;
pub mod auth;
pub mod handlers;

use std::sync::{Arc, Mutex};

use auth::{sign_in, initial_sign_up};
use storage::SecureStorage;
use handlers::{create_client, list_clients};
use tokio::sync::OnceCell;

/// Global storage reference, accessible only after login
pub static SECURE_STORAGE: OnceCell<Arc<Mutex<Option<SecureStorage>>>> = OnceCell::const_new();

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            initial_sign_up,
            sign_in,
            create_client,
            list_clients
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
