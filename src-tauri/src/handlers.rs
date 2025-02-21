use rusqlite::params;
use serde::{Deserialize, Serialize};
use crate::auth::SECURE_STORAGE;

/// Validate if the database is unlocked
fn get_db_conn() -> Result<rusqlite::Connection, String> {
    let storage = SECURE_STORAGE.get()
        .and_then(|s| s.lock().ok())
        .and_then(|guard| guard.as_ref().map(|s| s.conn.clone()));

    storage.ok_or("🔒 Database is locked. Please sign in first.".to_string())
}

/// ✅ Structs for API handlers
#[derive(Serialize, Deserialize)]
pub struct Client {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

/// 🏷️ Create a new client
#[tauri::command]
pub fn create_client(client: Client) -> Result<(), String> {
    let conn = get_db_conn()?;
    conn.execute(
        "INSERT INTO clients (name, email, phone) VALUES (?1, ?2, ?3)",
        params![client.name, client.email, client.phone]
    ).map_err(|e| e.to_string())?;

    Ok(())
}

/// 📋 List all clients
#[tauri::command]
pub fn list_clients() -> Result<Vec<Client>, String> {
    let conn = get_db_conn()?;
    let mut stmt = conn.prepare("SELECT id, name, email, phone FROM clients").map_err(|e| e.to_string())?;
    let clients_iter = stmt.query_map([], |row| {
        Ok(Client {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            phone: row.get(3)?,
        })
    }).map_err(|e| e.to_string())?;

    let clients: Vec<Client> = clients_iter.filter_map(Result::ok).collect();
    Ok(clients)
}

/// 📌 Event Struct
#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: Option<i32>,
    pub title: String,
    pub start_date: String,  // Store as `YYYY-MM-DD HH:MM:SS`
    pub end_date: String,
    pub client_id: Option<i32>,
}

/// 🗓️ Create an event
#[tauri::command]
pub fn create_event(event: Event) -> Result<(), String> {
    let conn = get_db_conn()?;

    conn.execute(
        "INSERT INTO events (title, start_date, end_date, client_id) VALUES (?1, ?2, ?3, ?4)",
        params![event.title, event.start_date, event.end_date, event.client_id]
    ).map_err(|e| e.to_string())?;

    Ok(())
}

/// ⏳ List all events
#[tauri::command]
pub fn list_events() -> Result<Vec<Event>, String> {
    let conn = get_db_conn()?;

    let mut stmt = conn.prepare("SELECT id, title, start_date, end_date, client_id FROM events")
        .map_err(|e| e.to_string())?;
    
    let events_iter = stmt.query_map([], |row| {
        Ok(Event {
            id: row.get(0)?,
            title: row.get(1)?,
            start_date: row.get(2)?,
            end_date: row.get(3)?,
            client_id: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?;

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
    pub status: String,  // "Paid", "Pending", "Overdue"
    pub event_id: Option<i32>,
}

/// 💵 Create an invoice
#[tauri::command]
pub fn create_invoice(invoice: Invoice) -> Result<(), String> {
    let conn = get_db_conn()?;

    conn.execute(
        "INSERT INTO invoices (client_id, amount, due_date, status, event_id) VALUES (?1, ?2, ?3, ?4, ?5)",
        params![invoice.client_id, invoice.amount, invoice.due_date, invoice.status, invoice.event_id]
    ).map_err(|e| e.to_string())?;

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
    pub status: String,  // "Scheduled" or "Posted"
}

/// 📢 Publish social media post
#[tauri::command]
pub fn schedule_social_post(post: SocialMediaPost) -> Result<(), String> {
    let conn = get_db_conn()?;

    conn.execute(
        "INSERT INTO social_media_posts (platform, content, schedule_time, event_id, client_id, status) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![post.platform, post.content, post.schedule_time, post.event_id, post.client_id, post.status]
    ).map_err(|e| e.to_string())?;

    Ok(())
}
