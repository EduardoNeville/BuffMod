pub mod supabase;
pub mod secure_db_access;
pub mod storage;
pub mod auth;

use auth::{sign_in, initial_sign_up};

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            initial_sign_up,
            sign_in,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
