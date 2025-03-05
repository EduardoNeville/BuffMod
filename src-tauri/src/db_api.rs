use std::path::PathBuf;
use rusqlite::{params, types::{FromSql, FromSqlResult, ToSqlOutput, ValueRef, FromSqlError}, Connection, ToSql};
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

    #[error("Encryption error: {0}")]
    EncryptionError(String),
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

    //match conn.execute(
    //    &format!("PRAGMA key = '{}'", encryption_key),
    //    params![]
    //) {
    //    Ok(_) => println!("Pragma key created..."),
    //    Err(err) => return Err(DbApiError::EncryptionError(err.to_string()))
    //}

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

/// 📄 Get a single client by ID
#[tauri::command]
pub fn get_client_by_id(state: tauri::State<StateWrapper>, client_id: i32) -> Result<Client, DbApiError> {
    let loc_state = state.lock().unwrap();
    let db_key = loc_state.as_ref().and_then(|s| s.db_key.clone()).unwrap();
    let db_path = loc_state.as_ref().and_then(|s| s.db_path.clone()).unwrap();
    let db_conn = open_encrypted_db(&db_path, &db_key)?;

    let mut stmt = db_conn.prepare("SELECT id, name, email, phone FROM clients WHERE id = ?1")?;
    let client_result = stmt.query_row([client_id], |row| {
        Ok(Client {
            id: row.get(0)?,
            name: row.get(1)?,
            email: row.get(2)?,
            phone: row.get(3)?,
        })
    });

    match client_result {
        Ok(client) => Ok(client),
        Err(_) => Err(DbApiError::SqliteError(rusqlite::Error::QueryReturnedNoRows)),
    }
}

/// 📌 Event Struct
#[derive(Serialize, Deserialize)]
pub struct Event {
    pub id: Option<i32>,
    pub kind: EventKind, // should be an enum
    pub title: String,
    pub schedule_time: String,  
    pub end_time: Option<String>,
    pub client_id: Option<i32>,
    pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub enum EventKind {
    Default,
    SocialMedia,
    Meeting,
}

// Implement ToSql for EventKind
impl ToSql for EventKind {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, rusqlite::Error> {
        let value = match self {
            EventKind::Default => "default",
            EventKind::SocialMedia => "socialmedia",
            EventKind::Meeting => "meeting",
        };
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(value.as_bytes())))
    }
}

impl FromSql for EventKind {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(bytes) => {
                match std::str::from_utf8(bytes) {
                    Ok("default") => Ok(EventKind::Default),
                    Ok("socialmedia") => Ok(EventKind::SocialMedia),
                    Ok("meeting") => Ok(EventKind::Meeting),
                    Ok(_other) => Err(FromSqlError::InvalidType), // Unexpected value
                    Err(_) => Err(FromSqlError::InvalidType),    // UTF-8 decoding failed
                }
            }
            _ => Err(FromSqlError::InvalidType), // Wrong SQL type
        }
    }
}

/// 🗓️ Create an event
pub fn create_event(db_conn: &Connection, event: Event) -> Result<i32, DbApiError> {
    db_conn.execute(
        "INSERT INTO events (kind, title, schedule_time, end_time, client_id, completed) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![event.kind, event.title, event.schedule_time, event.end_time, event.client_id, event.completed]
    )?;

    let event_id = db_conn.last_insert_rowid() as i32;

    Ok(event_id)
}

/// ⏳ List all events
#[tauri::command]
pub fn list_events(state: tauri::State<StateWrapper>) -> Result<Vec<Event>, DbApiError> {
    let loc_state = state.lock().unwrap(); 
    let db_key = loc_state.as_ref().and_then(|s| s.db_key.clone()).unwrap();
    let db_path = loc_state.as_ref().and_then(|s| s.db_path.clone()).unwrap();
    let db_conn = open_encrypted_db(&db_path, &db_key)?;
    let mut stmt = db_conn.prepare("SELECT id, title, start_date, end_time, client_id FROM events")?;
    
    let events_iter = stmt.query_map([], |row| {
        Ok(Event {
            id: row.get(0)?,
            kind: row.get(1)?,
            title: row.get(2)?,
            schedule_time: row.get(3)?,
            end_time: row.get(4)?,
            client_id: row.get(5)?,
            completed: row.get(6)?,
        })
    })?;

    let events: Vec<Event> = events_iter.filter_map(Result::ok).collect();
    Ok(events)
}

/// ⏳ List all events
#[tauri::command]
pub fn list_event_kind(state: tauri::State<StateWrapper>, event_kind: EventKind) -> Result<Vec<Event>, DbApiError> {
    let loc_state = state.lock().unwrap(); 
    let db_key = loc_state.as_ref().and_then(|s| s.db_key.clone()).unwrap();
    let db_path = loc_state.as_ref().and_then(|s| s.db_path.clone()).unwrap();
    let db_conn = open_encrypted_db(&db_path, &db_key)?;

    let mut stmt = db_conn.prepare(
        "SELECT id, kind, title, schedule_time, end_time, client_id, completed 
         FROM events 
         WHERE kind = ?1"
    )?;
    
    let events_iter = stmt.query_map(params![event_kind], |row| {
        Ok(Event {
            id: row.get(0)?,
            kind: row.get(1)?,
            title: row.get(2)?,
            schedule_time: row.get(3)?,
            end_time: row.get(4)?,
            client_id: row.get(5)?,
            completed: row.get(6)?,
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
    pub event_id: Option<i32>,
    pub platform: String,
    pub content: String,
    pub status: SocialMediaStatus,  
}

#[derive(Serialize, Deserialize)]
pub enum SocialMediaStatus {
    Drafted,
    Scheduled,
    Posted
}

// Implement ToSql for SocialMediaStatus
impl ToSql for SocialMediaStatus {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, rusqlite::Error> {
        let value = match self {
            SocialMediaStatus::Drafted => "drafted",
            SocialMediaStatus::Scheduled => "scheduled",
            SocialMediaStatus::Posted => "posted",
        };
        Ok(ToSqlOutput::Borrowed(ValueRef::Text(value.as_bytes())))
    }
}

impl FromSql for SocialMediaStatus{
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(bytes) => {
                match std::str::from_utf8(bytes) {
                    Ok("drafted") => Ok(SocialMediaStatus::Drafted),
                    Ok("scheduled") => Ok(SocialMediaStatus::Scheduled),
                    Ok("posted") => Ok(SocialMediaStatus::Posted),
                    Ok(_other) => Err(FromSqlError::InvalidType), // Unexpected value
                    Err(_) => Err(FromSqlError::InvalidType),    // UTF-8 decoding failed
                }
            }
            _ => Err(FromSqlError::InvalidType), // Wrong SQL type
        }
    }
}


/// 📢 Publish social media post
#[derive(Deserialize)]
pub struct ScheduleSocialPostArgs {
    pub post: SocialMediaPost,
    pub schedule_time: String,
}
#[tauri::command]
pub fn schedule_social_post(state: tauri::State<StateWrapper>, args: ScheduleSocialPostArgs) -> Result<(), DbApiError> {
    let loc_state = state.lock().unwrap();
    let db_key = loc_state.as_ref().and_then(|s| s.db_key.clone()).unwrap();
    let db_path = loc_state.as_ref().and_then(|s| s.db_path.clone()).unwrap();
    let db_conn = open_encrypted_db(&db_path, &db_key)?;

    // Extract from the args
    let ScheduleSocialPostArgs { post, schedule_time } = args;

    let event_id: Option<i32> = match post.status {
        SocialMediaStatus::Drafted => {
            None
        },
        SocialMediaStatus::Scheduled | SocialMediaStatus::Posted => {
            let event = Event {
                id: None,
                kind: EventKind::SocialMedia,
                title: format!(
                    "Social Media Post for ({})",
                    post.platform.replace("::", ", ")
                ),
                schedule_time,
                end_time: None,
                client_id: None,
                completed: match post.status {
                    SocialMediaStatus::Scheduled => false,
                    SocialMediaStatus::Posted => true,
                    _ => false // Case Draft but can't be due to previous match
                },
            };

            let id = create_event(&db_conn, event)?;
            Some(id)
        },
    };
    
    // Updated SQL to match the struct fields
    db_conn.execute(
        "INSERT INTO social_media_posts (event_id, platform, content, status) 
         VALUES (?1, ?2, ?3, ?4)",
        params![event_id, post.platform, post.content, post.status]
    )?;

    // Create the posts if there is one
    
    Ok(())
}



#[derive(Serialize)]
pub struct SocialMediaPostWithEvent {
    pub id: i32,
    pub event_id: Option<i32>,        // Can be None if no event is linked
    pub platform: String,
    pub content: String,
    pub status: String,
    pub schedule_time: Option<String> // Can be None if no event exists
}

#[tauri::command]
pub fn list_social_posts(state: tauri::State<StateWrapper>) -> Result<Vec<SocialMediaPostWithEvent>, DbApiError> {
    let loc_state = state.lock().unwrap();
    let db_key = loc_state.as_ref().and_then(|s| s.db_key.clone()).unwrap();
    let db_path = loc_state.as_ref().and_then(|s| s.db_path.clone()).unwrap();
    let db_conn = open_encrypted_db(&db_path, &db_key)?;

    // Use LEFT JOIN to include posts without an event_id
    let mut stmt = db_conn.prepare("
        SELECT p.id, p.event_id, p.platform, p.content, p.status, e.schedule_time 
        FROM social_media_posts p 
        LEFT JOIN events e ON p.event_id = e.id
        WHERE p.status='scheduled' OR p.status='posted'
    ")?;

    let posts_iter = stmt.query_map([], |row| {
        Ok(SocialMediaPostWithEvent {
            id: row.get(0)?,
            event_id: row.get(1)?,       // Will be None if NULL in DB
            platform: row.get(2)?,
            content: row.get(3)?,
            status: row.get(4)?,
            schedule_time: row.get(5)?,  // Will be None if no event
        })
    })?;

    let posts: Vec<SocialMediaPostWithEvent> = posts_iter.filter_map(Result::ok).collect();
    Ok(posts)
}


