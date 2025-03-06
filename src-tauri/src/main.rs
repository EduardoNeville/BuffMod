pub mod auth;
pub mod db_api;
pub mod secure_db_access;
pub mod storage;
pub mod supabase;

use std::sync::Mutex;
use rusqlite::Connection;
use tauri::Manager;

use tauri_plugin_stronghold;

pub struct AppState {
    pub db_conn: Option<Connection>,
}

type StateWrapper = Mutex<Option<AppState>>;

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle()
                .plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
            Ok(())
        })
        .manage(Mutex::new(None::<AppState>)) // Initialize state as None
        .invoke_handler(tauri::generate_handler![
            auth::initial_sign_up,
            auth::sign_in,
            db_api::create_client,
            db_api::list_clients,
            db_api::get_client_by_id,
            db_api::schedule_social_post,
            db_api::list_social_posts,
            db_api::list_event_kind,
            db_api::retrieve_post_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
