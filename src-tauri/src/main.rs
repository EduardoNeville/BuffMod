pub mod supabase;
pub mod secure_db_access;
pub mod storage;

use supabase::Supabase;

#[tauri::command]
async fn initial_sign_up(email: String, password: String, org_name: String, user_name: String) -> Result<(), String> {
    let supabase = Supabase::new();
    match supabase.initial_sign_up(&email, &password, &org_name, &user_name).await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("initial_sign_up failed: {}", e);
            Err(e) // Send error back to frontend
        }
    }
}

#[tauri::command]
async fn invite_user(org_id: String, email: String) -> Result<String, String> {
    let supabase = Supabase::new();
    match supabase.create_invite(&org_id, &email).await {
        Ok(invite_code) => Ok(invite_code),
        Err(e) => {
            eprintln!("invite_user failed: {}", e);
            Err(e) // Send error back to frontend
        }
    }
}

#[tauri::command]
async fn invite_sign_up(email: String, password: String, invite_code: String, user_name: String) -> Result<(), String> {
    let supabase = Supabase::new();
    match supabase.invite_sign_up(&email, &password, &invite_code, &user_name).await {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("invite_sign_up failed: {}", e);
            Err(e) // Send error back to frontend
        }
    }
}

#[tauri::command]
async fn sign_in(email: String, password: String) -> Result<Vec<String>, String> {
    let supabase = Supabase::new();
    supabase.sign_in(&email, &password).await
}

#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            initial_sign_up,
            invite_user,
            invite_sign_up,
            sign_in
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
