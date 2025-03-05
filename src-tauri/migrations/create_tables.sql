CREATE TABLE IF NOT EXISTS clients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    phone TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    kind TEXT NOT NULL,  -- e.g., 'social_media_post', 'meeting', etc.
    title TEXT NOT NULL,
    schedule_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP,
    client_id INTEGER,
    description TEXT,
    completed BOOLEAN,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS invoices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id INTEGER,
    amount REAL NOT NULL,
    due_date TIMESTAMP NOT NULL,
    status TEXT CHECK (status IN ('Paid', 'Pending', 'Overdue')) NOT NULL DEFAULT 'Pending',
    event_id INTEGER,
    invoice_number TEXT UNIQUE, 
    payment_date TIMESTAMP,     
    notes TEXT,                 
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE SET NULL,
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS expenses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    amount REAL NOT NULL,
    category TEXT NOT NULL,
    date TIMESTAMP NOT NULL,
    event_id INTEGER,
    description TEXT, 
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS social_media_posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER,
    platform TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT CHECK (status IN ('drafted', 'scheduled', 'posted')) NOT NULL DEFAULT 'drafted',
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS permissions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_id INTEGER NOT NULL,
    role TEXT CHECK (role IN ('admin', 'editor', 'viewer')) NOT NULL,
    last_updated TIMESTAMP DEFAULT CURRENT_TIMESTAMP 
);

CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    key TEXT UNIQUE NOT NULL,
    value TEXT,
    description TEXT 
);

CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id INTEGER NOT NULL,          -- Required link to clients
    event_id INTEGER,                    -- Optional link to events
    invoice_id INTEGER,                  -- Optional link to invoices
    name TEXT NOT NULL,                  -- Document name (e.g., 'Contract_2023.pdf')
    category TEXT,                       -- Optional category (e.g., 'Contract', 'Receipt')
    mime_type TEXT DEFAULT 'application/pdf', -- MIME type, defaulting to PDF
    size INTEGER,                        -- Size in bytes
    description TEXT,                    -- Optional notes or description
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    data BLOB,                           -- Binary data (PDF)
    FOREIGN KEY (client_id) REFERENCES clients(id) ON DELETE CASCADE,
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE SET NULL,
    FOREIGN KEY (invoice_id) REFERENCES invoices(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    social_media_post_id INTEGER,
    data BLOB,  -- Binary data (Image / Video)
    FOREIGN KEY (social_media_post_id) REFERENCES social_media_posts(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS meetings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER NOT NULL UNIQUE,
    location TEXT,
    attendees TEXT,
    -- Add other meeting-specific fields here
    FOREIGN KEY (event_id) REFERENCES events(id) ON DELETE CASCADE
);

-- Optional index for performance
--CREATE INDEX idx_documents_client_event ON documents (client_id, event_id);
--CREATE INDEX idx_posts_social_media_posts ON posts (social_media_posts_id);
