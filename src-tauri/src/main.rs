pub mod auth;
pub mod handlers;
pub mod secure_db_access;
pub mod storage;
pub mod supabase;

use std::sync::{Arc, Mutex};

use auth::{initial_sign_up, sign_in};
use handlers::{create_client, list_clients};
use storage::SecureStorage;
use tokio::sync::OnceCell;

/// Global storage reference, accessible only after login
pub static SECURE_STORAGE: OnceCell<Arc<Mutex<Option<SecureStorage>>>> = OnceCell::const_new();

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_stronghold::Builder::new(|pass| todo!()).build())
        .invoke_handler(tauri::generate_handler![
            initial_sign_up,
            sign_in,
            create_client,
            list_clients
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
