pub mod supabase;
pub mod storage;
pub mod auth;
pub mod handlers;
pub mod secure_db_access;

use auth::{sign_in, initial_sign_up};
use handlers::{create_client, list_clients};
use tauri::Manager;
use tauri_plugin_stronghold::stronghold::Stronghold;
use std::sync::Mutex;

pub struct AppState {
    stronghold: Mutex<Option<Stronghold>>,
    db_key: Mutex<Option<String>>,
}

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .manage(AppState {
            stronghold: Mutex::new(None),
            db_key: Mutex::new(None),
        })
        .invoke_handler(tauri::generate_handler![
            initial_sign_up,
            sign_in,
            create_client,
            list_clients
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
