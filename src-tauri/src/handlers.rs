use std::sync::MutexGuard;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use crate::{auth::SECURE_STORAGE, storage::SecureStorage};
use thiserror::Error;

/// Define a custom HandlerError enum for improved error handling
#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("🔒 Database is locked. Please sign in first.")]
    DatabaseLocked,

    #[error("🔑 Cannot acquire the database lock.")]
    DatabaseLockError,

    #[error("❌ Database connection is missing.")]
    DatabaseConnectionNotFound,

    #[error("🛑 SQLite Error: {0}")]
    SqliteError(#[from] rusqlite::Error),
}

// Implement serialization so we can return errors in Tauri commands
impl serde::Serialize for HandlerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

/// Validate if the database is unlocked and return a mutable reference
fn get_secure_storage() -> Result<MutexGuard<'static, Option<SecureStorage>>, HandlerError> {
    let storage_mutex = SECURE_STORAGE.get().ok_or(HandlerError::DatabaseLocked)?;
    
    let storage_guard = storage_mutex.lock().map_err(|_| HandlerError::DatabaseLockError)?;
    
    if storage_guard.is_none() {
        return Err(HandlerError::DatabaseConnectionNotFound);
    }

    Ok(storage_guard)
}

/// ✅ Client Struct
#[derive(Serialize, Deserialize)]
pub struct Client {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
}

/// 🏷️ Create a new client
#[tauri::command]
pub fn create_client(client: Client) -> Result<(), HandlerError> {
    let mut storage_guard = get_secure_storage()?;
    let storage = storage_guard.as_mut().ok_or(HandlerError::DatabaseConnectionNotFound)?;

    if let Some(conn) = &storage.conn {
        conn.execute(
            "INSERT INTO clients (name, email, phone) VALUES (?1, ?2, ?3)",
            params![client.name, client.email, client.phone]
        )?;

        Ok(())
    } else {
        Err(HandlerError::DatabaseConnectionNotFound)
    }
}

/// 📋 List all clients
#[tauri::command]
pub fn list_clients() -> Result<Vec<Client>, HandlerError> {
    let storage_guard = get_secure_storage()?;
    let storage = storage_guard.as_ref().ok_or(HandlerError::DatabaseConnectionNotFound)?;

    if let Some(conn) = &storage.conn {
        let mut stmt = conn.prepare("SELECT id, name, email, phone FROM clients")?;
        
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
    } else {
        Err(HandlerError::DatabaseConnectionNotFound)
    }
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
pub fn create_event(event: Event) -> Result<(), HandlerError> {
    let storage_guard = get_secure_storage()?;
    let storage = storage_guard.as_ref().ok_or(HandlerError::DatabaseConnectionNotFound)?;

    if let Some(conn) = &storage.conn {
        conn.execute(
            "INSERT INTO events (title, start_date, end_date, client_id) VALUES (?1, ?2, ?3, ?4)",
            params![event.title, event.start_date, event.end_date, event.client_id]
        )?;

        Ok(())
    } else {
        Err(HandlerError::DatabaseConnectionNotFound)
    }
}

/// ⏳ List all events
#[tauri::command]
pub fn list_events() -> Result<Vec<Event>, HandlerError> {
    let storage_guard = get_secure_storage()?;
    let storage = storage_guard.as_ref().ok_or(HandlerError::DatabaseConnectionNotFound)?;

    if let Some(conn) = &storage.conn {
        let mut stmt = conn.prepare("SELECT id, title, start_date, end_date, client_id FROM events")?;
        
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
    } else {
        Err(HandlerError::DatabaseConnectionNotFound)
    }
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
pub fn create_invoice(invoice: Invoice) -> Result<(), HandlerError> {
    let mut storage_guard = get_secure_storage()?;
    let storage = storage_guard.as_mut().ok_or(HandlerError::DatabaseConnectionNotFound)?;

    if let Some(conn) = &storage.conn {
        conn.execute(
            "INSERT INTO invoices (client_id, amount, due_date, status, event_id) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![invoice.client_id, invoice.amount, invoice.due_date, invoice.status, invoice.event_id]
        )?;

        Ok(())
    } else {
        Err(HandlerError::DatabaseConnectionNotFound)
    }
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
pub fn schedule_social_post(post: SocialMediaPost) -> Result<(), HandlerError> {
    let mut storage_guard = get_secure_storage()?;
    let storage = storage_guard.as_mut().ok_or(HandlerError::DatabaseConnectionNotFound)?;

    if let Some(conn) = &storage.conn {
        conn.execute(
            "INSERT INTO social_media_posts (platform, content, schedule_time, event_id, client_id, status) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![post.platform, post.content, post.schedule_time, post.event_id, post.client_id, post.status]
        )?;

        Ok(())
    } else {
        Err(HandlerError::DatabaseConnectionNotFound)
    }
}
