pub mod supabase;
pub mod storage;
pub mod auth;
pub mod db_api;
pub mod secure_db_access;

use tauri::Manager;
use std::{path::PathBuf, sync::Mutex};

use tauri_plugin_stronghold;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    pub db_key: Option<String>,
    pub db_path: Option<PathBuf>,
}

type StateWrapper = Mutex<Option<AppState>>;

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
        .manage(Mutex::new(None::<AppState>)) // Initialize state as None
        .invoke_handler(tauri::generate_handler![
            auth::initial_sign_up,
            auth::sign_in,
            db_api::create_client,
            db_api::list_clients
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


