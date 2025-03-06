use base64::decode;
use flate2::{read::GzDecoder, write::GzEncoder};
use flate2::Compression;
use rusqlite::{
    params,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
    Connection, ToSql,
};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::{fs::File, io::Write};
use std::io::Read;
use std::path::PathBuf;
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

    #[error("File error: {0}")]
    FileError(String),

    #[error("Failed to finish compression: {0}")]
    CompressionError(String),
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
pub fn open_encrypted_db(
    db_path: &PathBuf,
    encryption_key: &str,
) -> Result<Connection, DbApiError> {
    let conn = Connection::open(db_path)
        .map_err(|e| DbApiError::SqliteError(e))?;

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
pub fn create_client(
    state: tauri::State<StateWrapper>,
    client: Client
) -> Result<(), DbApiError> {
    // Extract connection from state
    let state_guard = state.lock().unwrap();
    let db_conn = state_guard.as_ref().and_then(|s| s.db_conn.as_ref()).unwrap();

    db_conn.execute(
        "INSERT INTO clients (name, email, phone) VALUES (?1, ?2, ?3)",
        params![client.name, client.email, client.phone],
    )?;

    Ok(())
}

/// 📋 List all clients
#[tauri::command]
pub fn list_clients(state: tauri::State<StateWrapper>) -> Result<Vec<Client>, DbApiError> {
    // Extract connection from state
    let loc_state = state.lock().unwrap();
    let db_conn = loc_state.as_ref().and_then(|s| s.db_conn.as_ref()).unwrap();
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
pub fn get_client_by_id(
    state: tauri::State<StateWrapper>,
    client_id: i32,
) -> Result<Client, DbApiError> {
    // Extract connection from state
    let loc_state = state.lock().unwrap();
    let db_conn = loc_state.as_ref().and_then(|s| s.db_conn.as_ref()).unwrap();

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
        Err(_) => Err(DbApiError::SqliteError(
            rusqlite::Error::QueryReturnedNoRows,
        )),
    }
}

/// 📌 Event Struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: Option<i32>,
    pub kind: EventKind, // should be an enum
    pub title: String,
    pub schedule_time: String,
    pub end_time: Option<String>,
    pub client_id: Option<i32>,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
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
                    Err(_) => Err(FromSqlError::InvalidType),     // UTF-8 decoding failed
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
    let db_conn = loc_state.as_ref().and_then(|s| s.db_conn.as_ref()).unwrap();

    let mut stmt =
        db_conn.prepare("SELECT id, title, start_date, end_time, client_id FROM events")?;

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
pub fn list_event_kind(
    state: tauri::State<StateWrapper>,
    event_kind: EventKind,
) -> Result<Vec<Event>, DbApiError> {
    // Extract connection from state
    let loc_state = state.lock().unwrap();
    let db_conn = loc_state.as_ref().and_then(|s| s.db_conn.as_ref()).unwrap();

    let mut stmt = db_conn.prepare(
        "SELECT id, kind, title, schedule_time, end_time, client_id, completed 
         FROM events 
         WHERE kind = ?1",
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
    println!("events: {:?}", events);
    Ok(events)
}

/// Events from a data period & kind
#[derive(Deserialize)]
pub struct ListEventsTimeKind {
    pub event_kind: EventKind,
    pub start_date: String,
    pub end_date: String,
}
#[tauri::command]
pub fn list_events_time_kind(
    state: tauri::State<StateWrapper>,
    args: ListEventsTimeKind,
) -> Result<Vec<Event>, DbApiError> {
    let loc_state = state.lock().unwrap();
    let db_conn = loc_state.as_ref().and_then(|s| s.db_conn.as_ref()).unwrap();

    let mut stmt = db_conn.prepare(
        "SELECT id, kind, title, schedule_time, end_time, client_id, completed 
         FROM events 
         WHERE kind = ?1 AND schedule_time>=?2 AND schedule_time <= ?3",
    )?;

    let events_iter = stmt.query_map(
        params![
            args.event_kind,
            args.start_date,
            args.end_date
        ], |row| {
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
pub fn create_invoice(
    state: tauri::State<StateWrapper>,
    invoice: Invoice,
) -> Result<(), DbApiError> {
    // Extract connection from state
    let loc_state = state.lock().unwrap();
    let db_conn = loc_state.as_ref().and_then(|s| s.db_conn.as_ref()).unwrap();
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
    Posted,
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

impl FromSql for SocialMediaStatus {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(bytes) => {
                match std::str::from_utf8(bytes) {
                    Ok("drafted") => Ok(SocialMediaStatus::Drafted),
                    Ok("scheduled") => Ok(SocialMediaStatus::Scheduled),
                    Ok("posted") => Ok(SocialMediaStatus::Posted),
                    Ok(_other) => Err(FromSqlError::InvalidType), // Unexpected value
                    Err(_) => Err(FromSqlError::InvalidType),     // UTF-8 decoding failed
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
    pub file_path: Option<String>,
}
#[tauri::command]
pub fn schedule_social_post(
    state: tauri::State<StateWrapper>,
    args: ScheduleSocialPostArgs,
) -> Result<(), DbApiError> {
    // Extract connection from state
    let loc_state = state.lock().unwrap();
    let db_conn = loc_state.as_ref().and_then(|s| s.db_conn.as_ref()).unwrap();

    // Extract from the args
    let ScheduleSocialPostArgs {
        post,
        schedule_time,
        file_path,
    } = args;

    let event_id: Option<i32> = match post.status {
        SocialMediaStatus::Drafted => None,
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
                    _ => false, // Draft case handled above
                },
            };
            let id = create_event(&db_conn, event)?;
            Some(id)
        }
    };

    // Insert into social_media_posts
    db_conn.execute(
        "INSERT INTO social_media_posts (event_id, platform, content, status) 
         VALUES (?1, ?2, ?3, ?4)",
        params![event_id, post.platform, post.content, post.status],
    )?;

    // Get the inserted social media post ID
    let social_media_post_id = db_conn.last_insert_rowid() as i32;

    // Handle file if provided
    println!("File_path: {:?}", &file_path);
    if let Some(file_path) = file_path {
        // Read the file
        let mut file = File::open(&file_path).map_err(|e| DbApiError::FileError(format!("File.open error: {}", e)))?;
        let mut file_data = Vec::new();
        file.read_to_end(&mut file_data)
            .map_err(|e| DbApiError::FileError(format!("file.read_to_end error: {}", e)))?;

        // Compress the file
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&file_data)
            .map_err(|e| DbApiError::CompressionError(format!("encoder.write_all error: {}", e)))?;
        let compressed_data = encoder
            .finish()
            .map_err(|e| DbApiError::CompressionError(format!("encoder.finish error: {}", e)))?;

        // Store compressed data
        db_conn
            .execute(
                "INSERT INTO posts (social_media_post_id, data) VALUES (?1, ?2)",
                params![social_media_post_id, compressed_data],
            )
            .map_err(|e| DbApiError::SqliteError(e))?;

        // Removing the temp file
        let _ = std::fs::remove_file(file_path)
            .map_err(|e| DbApiError::FileError(format!("File.remove error: {}", e)))?;
    }

    Ok(())
}

#[derive(Serialize)]
pub struct SocialMediaPostWithEvent {
    pub id: i32,
    pub event_id: Option<i32>, // Can be None if no event is linked
    pub platform: String,
    pub content: String,
    pub status: String,
    pub schedule_time: Option<String>, // Can be None if no event exists
}

#[tauri::command]
pub fn list_social_posts(
    state: tauri::State<StateWrapper>,
) -> Result<Vec<SocialMediaPostWithEvent>, DbApiError> {
    // Extract connection from state
    let loc_state = state.lock().unwrap();
    let db_conn = loc_state.as_ref().and_then(|s| s.db_conn.as_ref()).unwrap();

    // Use LEFT JOIN to include posts without an event_id
    let mut stmt = db_conn.prepare(
        "
        SELECT p.id, p.event_id, p.platform, p.content, p.status, e.schedule_time 
        FROM social_media_posts p 
        LEFT JOIN events e ON p.event_id = e.id
        WHERE p.status='scheduled' OR p.status='posted'
    ",
    )?;

    let posts_iter = stmt.query_map([], |row| {
        Ok(SocialMediaPostWithEvent {
            id: row.get(0)?,
            event_id: row.get(1)?, // Will be None if NULL in DB
            platform: row.get(2)?,
            content: row.get(3)?,
            status: row.get(4)?,
            schedule_time: row.get(5)?, // Will be None if no event
        })
    })?;

    let posts: Vec<SocialMediaPostWithEvent> = posts_iter.filter_map(Result::ok).collect();
    Ok(posts)
}

#[tauri::command]
pub fn retrieve_post_file(
    state: tauri::State<'_, StateWrapper>,
    app_handle: tauri::AppHandle,
    social_media_post_id: i32,
) -> Result<String, DbApiError> {
    // Extract connection from state
    let loc_state = state.lock().unwrap();
    let db_conn = loc_state.as_ref().and_then(|s| s.db_conn.as_ref()).unwrap();

    // Query the posts table for the compressed data
    let mut stmt = db_conn.prepare(
        "SELECT data FROM posts WHERE social_media_post_id = ?1 LIMIT 1"
    )?;
    let mut rows = stmt.query(params![social_media_post_id])?;
    let row = rows.next()?.unwrap();
    let compressed_data: Vec<u8> = row.get(0)?;

    // Decompress the data
    let mut decoder = GzDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).unwrap();

    // Determine the file type (basic approach with default extension)
    let file_extension = {
        let kind = infer::get(&decompressed_data);
        match kind {
            Some(k) => k.extension(),
            None => "bin",
        }
    };

    // Construct the output file path in dataDir()/buffmod/
    let data_dir = app_handle.path().data_dir()
        .map_err(|e| DbApiError::FileError(format!("Failed to resolve app data dir: {}", e)))?;
    let output_dir = data_dir.join("buffmod/tmp");
    std::fs::create_dir_all(&output_dir).map_err(|e| {
        DbApiError::FileError(format!("Failed to create directory {}: {}", output_dir.display(), e))
    })?;
    let output_file_path = output_dir.join(format!("post_{}.{}", social_media_post_id, file_extension));

    // Write the decompressed data to a file
    let mut output_file = File::create(&output_file_path).map_err(|e| {
        DbApiError::FileError(format!("Failed to create file {}: {}", output_file_path.display(), e))
    })?;
    output_file.write_all(&decompressed_data).map_err(|e| {
        DbApiError::FileError(format!("Failed to write to file {}: {}", output_file_path.display(), e))
    })?;

    // Return the file path as a string for the frontend
    Ok(output_file_path.to_string_lossy().into_owned())
}
