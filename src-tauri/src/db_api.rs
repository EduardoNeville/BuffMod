use std::path::PathBuf;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::StateWrapper;

/// Define a custom DbApiError enum for improved error handling
#[derive(Debug, Error)]
pub enum DbApiError {
    #[error("🔒 Database is locked. Please sign in first.")]
    DatabaseLocked,

    #[error("🔑 Cannot acquire the database lock.")]
    DatabaseLockError,

    #[error("❌ Database connection is missing.")]
    DatabaseConnectionNotFound,

    #[error("🛑 SQLite Error: {0}")]
    SqliteError(#[from] rusqlite::Error),

    #[error("Encryption error")]
    EncryptionError,
}

// Implement serialization so we can return errors in Tauri commands
impl serde::Serialize for DbApiError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

/// ✅ Client Struct
#[derive(Serialize, Deserialize)]
pub struct Client {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}


/// Opens an SQLite encrypted database
pub fn open_encrypted_db(db_path: &PathBuf, encryption_key: &str) -> Result<Connection, DbApiError> {
    let conn = Connection::open(db_path).map_err(|e| DbApiError::SqliteError(e))?;

    conn.execute(&format!("PRAGMA key = '{}'", encryption_key), [])
        .map_err(|_| DbApiError::EncryptionError)?;

    Ok(conn)
}

/// 🏷️ Create a new client
#[tauri::command]
pub fn create_client(state: tauri::State<StateWrapper>, client: Client) -> Result<(), DbApiError> {
    let state_guard = state.lock().unwrap();
    let db_path = state_guard.as_ref().and_then(|s| s.db_path.clone()).unwrap();
    let db_key = state_guard.as_ref().and_then(|s| s.db_key.clone()).unwrap();

    let db_conn = open_encrypted_db(&db_path, &db_key)?;
    db_conn.execute(
        "INSERT INTO clients (name, email, phone) VALUES (?1, ?2, ?3)",
        params![client.name, client.email, client.phone]
    )?;

    Ok(())
}

/// 📋 List all clients
#[tauri::command]
pub fn list_clients(state: tauri::State<StateWrapper>) -> Result<Vec<Client>, DbApiError> {
    let loc_state = state.lock().unwrap();
    let db_key = loc_state.as_ref().and_then(|s| s.db_key.clone()).unwrap();
    let db_path = loc_state.as_ref().and_then(|s| s.db_path.clone()).unwrap();
    let db_conn = open_encrypted_db(&db_path, &db_key)?;
    let mut stmt = db_conn.prepare("SELECT id, name, email, phone FROM clients")?;
    
    let clients_iter = stmt.query_map([], |row| {
        Ok(Client {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            phone: row.get(3)?,
        })
    })?;

    let clients: Vec<Client> = clients_iter.filter_map(Result::ok).collect();
    Ok(clients)
}

/// 📌 Event Struct
#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: Option<i32>,
    pub title: String,
    pub start_date: String,  
    pub end_date: String,
    pub client_id: Option<i32>,
}

/// 🗓️ Create an event
#[tauri::command]
pub fn create_event(state: tauri::State<StateWrapper>, event: Event) -> Result<(), DbApiError> {
    let loc_state = state.lock().unwrap(); 
    let db_key = loc_state.as_ref().and_then(|s| s.db_key.clone()).unwrap();
    let db_path = loc_state.as_ref().and_then(|s| s.db_path.clone()).unwrap();
    let db_conn = open_encrypted_db(&db_path, &db_key)?;
    db_conn.execute(
        "INSERT INTO events (title, start_date, end_date, client_id) VALUES (?1, ?2, ?3, ?4)",
        params![event.title, event.start_date, event.end_date, event.client_id]
    )?;

    Ok(())
}

/// ⏳ List all events
#[tauri::command]
pub fn list_events(state: tauri::State<StateWrapper>) -> Result<Vec<Event>, DbApiError> {
    let loc_state = state.lock().unwrap(); 
    let db_key = loc_state.as_ref().and_then(|s| s.db_key.clone()).unwrap();
    let db_path = loc_state.as_ref().and_then(|s| s.db_path.clone()).unwrap();
    let db_conn = open_encrypted_db(&db_path, &db_key)?;
    let mut stmt = db_conn.prepare("SELECT id, title, start_date, end_date, client_id FROM events")?;
    
    let events_iter = stmt.query_map([], |row| {
        Ok(Event {
            id: row.get(0)?,
            title: row.get(1)?,
            start_date: row.get(2)?,
            end_date: row.get(3)?,
            client_id: row.get(4)?,
        })
    })?;

    let events: Vec<Event> = events_iter.filter_map(Result::ok).collect();
    Ok(events)
}

/// 🧾 Invoice Struct
#[derive(Serialize, Deserialize)]
pub struct Invoice {
    pub id: Option<i32>,
    pub client_id: i32,
    pub amount: f64,
    pub due_date: String,
    pub status: String,  
    pub event_id: Option<i32>,
}

/// 💵 Create an invoice
#[tauri::command]
pub fn create_invoice(state: tauri::State<StateWrapper>, invoice: Invoice) -> Result<(), DbApiError> {
    let loc_state = state.lock().unwrap(); 
    let db_key = loc_state.as_ref().and_then(|s| s.db_key.clone()).unwrap();
    let db_path = loc_state.as_ref().and_then(|s| s.db_path.clone()).unwrap();
    let db_conn = open_encrypted_db(&db_path, &db_key)?;
    db_conn.execute(
        "INSERT INTO invoices (client_id, amount, due_date, status, event_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![invoice.client_id, invoice.amount, invoice.due_date, invoice.status, invoice.event_id]
    )?;
    Ok(())

}

/// 📲 Social Media Struct
#[derive(Serialize, Deserialize)]
pub struct SocialMediaPost {
    pub id: Option<i32>,
    pub platform: String,
    pub content: String,
    pub schedule_time: String,
    pub event_id: Option<i32>,
    pub client_id: Option<i32>,
    pub status: String,  
}

/// 📢 Publish social media post
#[tauri::command]
pub fn schedule_social_post(state: tauri::State<StateWrapper>, post: SocialMediaPost) -> Result<(), DbApiError> {
    let loc_state = state.lock().unwrap(); 
    let db_key = loc_state.as_ref().and_then(|s| s.db_key.clone()).unwrap();
    let db_path = loc_state.as_ref().and_then(|s| s.db_path.clone()).unwrap();
    let db_conn = open_encrypted_db(&db_path, &db_key)?;
    db_conn.execute(
        "INSERT INTO social_media_posts (platform, content, schedule_time, event_id, client_id, status) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![post.platform, post.content, post.schedule_time, post.event_id, post.client_id, post.status]
    )?;

    Ok(())
}
