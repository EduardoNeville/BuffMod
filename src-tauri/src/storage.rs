use rusqlite::Connection;
use crate::secure_db_access::EncKey;


pub struct SecureStorage {
    conn: Connection
}

impl SecureStorage {
    pub fn new(db_path: &str, enc_key: EncKey) -> Self {
        // Generated encrypted key to open db
        let gen_key = enc_key.generate_enc_key();
        let decrypt_conn = Self::open_encrypted_db(db_path, &gen_key).unwrap();

        SecureStorage {
            conn: decrypt_conn
        }
    }

    /// Open a database with encryption using the derived key
    fn open_encrypted_db(db_path: &str, encryption_key: &[u8]) -> Result<Connection, rusqlite::Error> {
        let conn = Connection::open(db_path)?;
        
        // Convert key to hex because SQLCipher expects a hexadecimal key
        let hex_key = hex::encode(encryption_key);
        
        // Apply encryption key
        conn.execute(&format!("PRAGMA key = '{}'", hex_key), [])?;
        
        Ok(conn)
    }

}
