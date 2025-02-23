pub mod supabase;
pub mod storage;
pub mod auth;
pub mod handlers;
pub mod secure_db_access;

use auth::{sign_in, initial_sign_up};
use handlers::{create_client, list_clients};
use storage::new_db;
use tauri::{AppHandle, Manager};
use tauri_plugin_stronghold::stronghold::Stronghold;
use std::sync::Mutex;

struct AppState {
    stronghold: Mutex<Option<Stronghold>>,
    db_key: Mutex<Option<String>>,
}

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

            Ok(())
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
