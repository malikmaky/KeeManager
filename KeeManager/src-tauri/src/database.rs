use chrono::DateTime;
use once_cell::sync::OnceCell;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    encode, 
    hash,
    encryption, 
    key_derivation,
    entry::{CreditCardEntry, IdentityEntry, LoginEntry, NoteEntry},
};

static mut DATABASE_INSTANCE: OnceCell<Arc<Mutex<Database>>> = OnceCell::new();

pub struct Database {
    conn: Connection,
    master_key: String,
}

impl Database {
    //
    // If the database exists it opens a connection
    // otherwise it creates a new database sets up the tables and stores the master key hash
    //
    fn new(db_path: &str, master_key: &str) -> Result<Self> {
        if Database::database_exists(db_path) {
            let conn = match Connection::open(db_path)
            {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("DB_get_instance: connection failure: {}", e);
                    return Err(e);
                }
            };
            Ok(Database {
                conn,
                master_key: master_key.to_string(),
            })
        } else {
            let conn = match Connection::open(db_path)
            {
                Ok(conn) => conn,
                Err(e) => {
                    eprintln!("DB_instance_creation: connection failure: {}", e);
                    return Err(e);
                }
            };

            Self::create_tables(&conn);
            
            if !Self::set_master_key_hash(&conn, master_key) {
                eprintln!("DB_instance_creation: set master key failure");
                return Err(rusqlite::Error::UnwindingPanic);
            }

            Ok(Database {
                conn,
                master_key: master_key.to_string(),
            })
        }
    }

    //
    // Singleton instance
    //
    pub fn get_instance(db_path: &str, master_key: &str) -> Arc<Mutex<Database>> {
        let clone: Arc<Mutex<Database>>;
        unsafe {
            if Database::database_exists(db_path) & DATABASE_INSTANCE.get().is_none() {
                encryption::decrypt_database(db_path, master_key);
            }
            clone = DATABASE_INSTANCE
                .get_or_init(|| {
                    Arc::new(Mutex::new(
                        Database::new(db_path, master_key)
                            .expect("Failed to initialize or get database instance"),
                    ))
                })
                .clone();
            return clone;
        }
    }

    //
    // Clears the singleton database instance
    //
    pub fn clear_instance() {
        unsafe {
            if DATABASE_INSTANCE.get().is_some() {
                let db_instance = DATABASE_INSTANCE.get().expect("Cloning instance failed").lock().expect("Locking Mutex failed");
                let db_path = db_instance.conn.path().unwrap();
                let master_key = db_instance.master_key.clone();

                eprintln!("Singleton instance cleared masterkey : {}.", master_key);
                let _ = encryption::encrypt_database(db_path, &master_key);
            } else {
                eprintln!("There was no singleton instance to clear.");
            }
            // Destroy singleton instance
            DATABASE_INSTANCE.take();
        }
    }

    fn set_master_key_hash(conn: &Connection, master_key: &str) -> bool {
        let decoded_master_key = match encode::decode_base64(master_key)
        {
            Ok(decoded_master_key) => decoded_master_key,
            Err(e) => {
                eprintln!("Set Master Key: master key decoding failure: {}", e);
                return false;
            }
        };

        let hashed_master_key = match hash::hash_sha256(decoded_master_key)
        {
            Ok(hashed_master_key) => hashed_master_key,
            Err(e) => {
                eprintln!("Set Master Key: master key hashing failure: {}", e);
                return false;
            }
        };

        let encoded_hashed_master_key =
            encode::encode_base64(hashed_master_key.as_bytes().to_vec());

        match conn.execute(
            "INSERT OR REPLACE INTO MasterKey (id, hashedMasterKey) VALUES (1, ?1);",
            params![encoded_hashed_master_key],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Set Master Key: SQL failure: {}", e);
                return false;
            }
        };

        true
    }

    pub fn save_master_key_hash(db_path: &str, master_key: &str) -> bool {
        let conn = match Connection::open(db_path)
        {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Save Master Key: connection failure: {}", e);
                return false;
            }
        };

        let decoded_master_key = match encode::decode_base64(master_key){
            Ok(decoded_master_key) => decoded_master_key,
            Err(e) => {
                eprintln!("Save Master Key: master key decoding failure: {}", e);
                return false;
            }
        };

        let hashed_master_key = match hash::hash_sha256(decoded_master_key){
            Ok(hashed_master_key) => hashed_master_key,
            Err(e) => {
                eprintln!("Save Master Key: master key hashing failure: {}", e);
                return false;
            }
        };

        let encoded_hashed_master_key =
            encode::encode_base64(hashed_master_key.as_bytes().to_vec());

        match conn.execute(
            "INSERT OR REPLACE INTO MasterKey (id, hashedMasterKey) VALUES (1, ?1);",
            params![encoded_hashed_master_key],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Save Master Key: SQL failure: {}", e);
                return false;
            }
        };

        true
    }

    pub fn check_master_key_hash(db_path: &str, master_key: &str) -> bool {
        if !encryption::decrypt_database(db_path, master_key) {
            return false;
        }

        let conn = match Connection::open(db_path){
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Check Master Key: connection failure: {}", e);
                return false;
            }
        };

        // Prepare the SQL statement
        let mut stmt = match conn
            .prepare("SELECT hashedMasterKey FROM MasterKey WHERE id = 1;")
            {
                Ok(stmt) => stmt,
                Err(e) => {
                    eprintln!("Check Master Key: SQL statement failure: {}", e);
                    return false;
                }
            };

        let mut rows = match stmt.query([])
        {
            Ok(rows) => rows,
            Err(e) => {
                eprintln!("Check Master Key: SQL query failure: {}", e);
                return false;
            }
        };
        
        if let Some(row) = rows.next().expect("Check Master Key: row query failure") {
            let saved_hashed_master_key: String = row.get(0).expect("Check Master Key: getting master key failed");

            let decoded_master_key = match encode::decode_base64(master_key)
            {
                Ok(decoded_master_key) => decoded_master_key,
                Err(e) => {
                    eprintln!("Check Master Key: master key decoding failure: {}", e);
                    return false;
                }
            };
            let hashed_master_key = match hash::hash_sha256(decoded_master_key){
                Ok(hashed_master_key) => hashed_master_key,
                Err(e) => {
                    eprintln!("Check Master Key: master key hashing failure: {}", e);
                    return false;
                }
            };

            let encoded_hashed_master_key =
            encode::encode_base64(hashed_master_key.as_bytes().to_vec());
            drop(rows);
            drop(stmt);
            drop(conn);
            let _ = encryption::encrypt_database(db_path, master_key);
            return saved_hashed_master_key == encoded_hashed_master_key;
        } else {
            let _ = encryption::encrypt_database(db_path, master_key);
            return false;
        }
    }

    pub fn database_exists(db_path: &str) -> bool {
        fs::metadata(db_path).is_ok()
    }

    fn create_tables(conn: &Connection) -> bool {
        match conn.execute(
            "CREATE TABLE IF NOT EXISTS MasterKey (
                    id INTEGER PRIMARY KEY CHECK (id = 1),
                    hashedMasterKey TEXT NOT NULL           
                )
            ",
            [],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Create tables: SQL MasterKey table failure: {}", e);
                return false;
            } 
        };

        match conn.execute(
            "CREATE TABLE IF NOT EXISTS LoginEntries (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    username TEXT NOT NULL,
                    password TEXT NOT NULL,
                    salt TEXT NOT NULL UNIQUE,
                    url TEXT,
                    notes TEXT,
                    last_updated DATETIME
                )
            ",
            [],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Create tables: SQL LoginEntries table failure: {}", e);
                return false;
            } 
        };

        match conn.execute(
            "CREATE TABLE IF NOT EXISTS CreditCardEntries (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    card_number TEXT NOT NULL,
                    expiry_date TEXT NOT NULL,
                    cardholder_name TEXT NOT NULL,
                    cvv TEXT NOT NULL,
                    salt TEXT NOT NULL UNIQUE,
                    last_updated DATETIME
                )
            ",
            [],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Create tables: SQL CreditCardEntries table failure: {}", e);
                return false;
            } 
        };

        match conn.execute(
            "CREATE TABLE IF NOT EXISTS NoteEntries (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    content TEXT NOT NULL,
                    salt TEXT NOT NULL UNIQUE,
                    last_updated DATETIME
                )
            ",
            [],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Create tables: SQL NoteEntries table failure: {}", e);
                return false;
            } 
        };

        match conn.execute(
            "CREATE TABLE IF NOT EXISTS IdentityEntries (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    full_name TEXT NOT NULL,
                    date_of_birth TEXT NOT NULL,
                    nationality TEXT NOT NULL,
                    identification_number TEXT NOT NULL UNIQUE,
                    issue_date TEXT NOT NULL,
                    expiry_date TEXT NOT NULL,
                    issuer TEXT NOT NULL,
                    notes TEXT,
                    salt TEXT NOT NULL UNIQUE,
                    last_updated DATETIME
                )
            ",
            [],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Create tables: SQL IdentityEntries table failure: {}", e);
                return false;
            } 
        };

        true
    }

    pub fn add_login(&self, entry: LoginEntry) -> bool {
        let salt = encryption::generate_salt();
        let derived_key = match key_derivation::derive_key(&self.master_key, &salt)
        {
            Ok(derived_key) => derived_key,
            Err(e) => {
                eprintln!("Add login: key derivation failed: {}", e);
                return false;
            }
        };

        let encrypted_title = match encryption::encrypt_gcm(&entry.title, &derived_key)
        {
            Ok(encrypted_title) => encrypted_title,
            Err(e) => {
                eprintln!("Add login: title encryption failed: {}", e);
                return false;
            } 
        };

        let encrypted_username =
            match encryption::encrypt_gcm(&entry.username, &derived_key){
            Ok(encrypted_username) => encrypted_username,
            Err(e) => {
                eprintln!("Add login: username encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_password =
            match encryption::encrypt_gcm(&entry.password, &derived_key){
                Ok(encrypted_password) => encrypted_password,
                Err(e) => {
                    eprintln!("Add login: password encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_url = match encryption::encrypt_gcm(&entry.url, &derived_key){
            Ok(encrypted_url) => encrypted_url,
            Err(e) => {
                eprintln!("Add login: url encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_notes = match encryption::encrypt_gcm(&entry.notes, &derived_key){
            Ok(encrypted_notes) => encrypted_notes,
            Err(e) => {
                eprintln!("Add login: note encryption failed: {}", e);
                return false;
            } 
        };

        match self.conn.execute(
            "INSERT INTO LoginEntries (title, username, password, salt, url, notes, last_updated) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                encrypted_title,
                encrypted_username,
                encrypted_password,
                salt,
                encrypted_url,
                encrypted_notes,
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
            ],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Add login: insertion failed: {}", e);
                return false;
            }
        };

        true
    }

    pub fn update_login(&self, entry: LoginEntry) -> bool {
        let salt = encryption::generate_salt();
        let derived_key = match key_derivation::derive_key(&self.master_key, &salt) {
            Ok(derived_key) => derived_key,
            Err(e) => {
                eprintln!("Update login: key derivation failed: {}", e);
                return false;
            }
        };
        let encrypted_title = match encryption::encrypt_gcm(&entry.title, &derived_key){
            Ok(encrypted_title) => encrypted_title,
            Err(e) => {
                eprintln!("Update login: title encryption failed: {}", e);
                return false;
            } 
        };

        let encrypted_username =
            match encryption::encrypt_gcm(&entry.username, &derived_key){
            Ok(encrypted_username) => encrypted_username,
            Err(e) => {
                eprintln!("Update login: username encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_password =
            match encryption::encrypt_gcm(&entry.password, &derived_key){
                Ok(encrypted_password) => encrypted_password,
                Err(e) => {
                    eprintln!("Update login: password encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_url = match encryption::encrypt_gcm(&entry.url, &derived_key){
            Ok(encrypted_url) => encrypted_url,
            Err(e) => {
                eprintln!("Update login: url encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_notes = match encryption::encrypt_gcm(&entry.notes, &derived_key){
            Ok(encrypted_notes) => encrypted_notes,
            Err(e) => {
                eprintln!("Update login: note encryption failed: {}", e);
                return false;
            } 
        };

        match self.conn.execute(
            "UPDATE LoginEntries 
             SET title = ?1, username = ?2, password = ?3, salt = ?4, url = ?5, notes = ?6, last_updated = ?7 
             WHERE id = ?8",
            params![
                encrypted_title,
                encrypted_username,
                encrypted_password,
                salt,
                encrypted_url,
                encrypted_notes,
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                entry.id
            ],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Update login: update failed: {}", e);
                return false;
            }
        };

        true
    }

    pub fn delete_login(&self, id: &str) -> bool {
        match self.conn
            .execute("DELETE FROM LoginEntries WHERE id = ?1", params![id])
            {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Delete Login: delete failed: {}", e);
                    return false;
            }
            };

        true
    }

    pub fn get_logins(&self) -> Result<Vec<LoginEntry>, bool> {
        let mut stmt = match self.conn.prepare(
            "SELECT id, title, username, password, salt, url, notes, last_updated FROM LoginEntries",
        ){
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Get login: connection statement failed: {}", e);
                return Err(false);
            } 
        };

        let entry_iter = match stmt.query_map([], |row| {
            let last_updated_datetime = row.get::<_, i64>(7).expect("Get Logins: getting last update time failed");
            let datetime = DateTime::from_timestamp(last_updated_datetime, 0).unwrap();
            // Format as "day-month-year"
            let last_updated_datetime_string = datetime.format("%d.%m.%Y %H:%M:%S").to_string();

            let encoded_salt: String = row.get::<_, String>(4).expect("Get Logins: getting salt failed");
            let derived_key = match key_derivation::derive_key(&self.master_key, &encoded_salt)
            {
                Ok(derived_key) => derived_key,
                Err(_) => {
                    eprintln!("Get logins: key derivation failure");
                    return Err(rusqlite::Error::UnwindingPanic);
                }
            };
    
            let decrypted_title =
                match encryption::decrypt_gcm(&row.get::<_, String>(1).expect("Get Logins: getting title failed"), &derived_key)
                {
                    Ok(decrypted_title) => decrypted_title,
                    Err(_) => {
                        eprintln!("Get logins: title decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
            let decrypted_username =
                match encryption::decrypt_gcm(&row.get::<_, String>(2).expect("Get Logins: getting username failed"), &derived_key)
                {
                    Ok(decrypted_username) => decrypted_username,
                    Err(_) => {
                        eprintln!("Get logins: username decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
            let decrypted_password =
                match encryption::decrypt_gcm(&row.get::<_, String>(3).expect("Get Logins: getting password failed"), &derived_key)
                {
                    Ok(decrypted_password) => decrypted_password,
                    Err(_) => {
                        eprintln!("Get logins: password decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
            let decrypted_url =
                match encryption::decrypt_gcm(&row.get::<_, String>(5).expect("Get Logins: getting url failed"), &derived_key)
                {
                    Ok(decrypted_url) => decrypted_url,
                    Err(_) => {
                        eprintln!("Get Logins: url decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
            let decrypted_notes =
                match encryption::decrypt_gcm(&row.get::<_, String>(6).expect("Get Logins: getting note failed"), &derived_key)
                {
                    Ok(decrypted_notes) => decrypted_notes,
                    Err(_) => {
                        eprintln!("Get Logins: note decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
            Ok(LoginEntry {
                id: row.get::<_, i64>(0).expect("Get Logins: getting id failed").to_string(),
                title: decrypted_title,
                username: decrypted_username,
                password: decrypted_password,
                url: decrypted_url,
                notes: decrypted_notes,
                last_updated: last_updated_datetime_string,
            })
        }){
            Ok(entry_iter) => entry_iter,
            Err(e) => {
                eprintln!("Get Logins: query failed: {}", e);
                return Err(false);
            } 
        };

        let mut entries = Vec::new();
        for entry in entry_iter {
            entries.push(entry.expect("Get Logins: pushing entries error"));
        }
        Ok(entries)
    }

    pub fn add_credit_card(&self, entry: CreditCardEntry) -> bool {
        let salt = encryption::generate_salt();
        let derived_key = match key_derivation::derive_key(&self.master_key, &salt) {
            Ok(derived_key) => derived_key,
            Err(e) => {
                eprintln!("Add Credit Card: key derivation failed: {}", e);
                return false;
            }
        };
        let encrypted_title = match encryption::encrypt_gcm(&entry.title, &derived_key)
        {
            Ok(encrypted_title) => encrypted_title,
            Err(e) => {
                eprintln!("Add Credit Card: title encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_card_number =
           match encryption::encrypt_gcm(&entry.card_number, &derived_key)
            {
                Ok(encrypted_card_number) => encrypted_card_number,
                Err(e) => {
                    eprintln!("Add Credit Card: card number encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_expiry_date =
            match encryption::encrypt_gcm(&entry.expiry_date, &derived_key)
            {
                Ok(encrypted_expiry_date) => encrypted_expiry_date,
                Err(e) => {
                    eprintln!("Add Credit Card: expiry date encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_cardholder_name =
            match encryption::encrypt_gcm(&entry.cardholder_name, &derived_key)
            {
                Ok(encrypted_cardholder_name) => encrypted_cardholder_name,
                Err(e) => {
                    eprintln!("Add Credit Card: cardholder name encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_cvv = match encryption::encrypt_gcm(&entry.cvv, &derived_key)
        {
            Ok(encrypted_cvv) => encrypted_cvv,
            Err(e) => {
                eprintln!("Add Credit Card: CVV encryption failed: {}", e);
                return false;
            } 
        };

        match self.conn.execute(
            "INSERT INTO CreditCardEntries (title, card_number, expiry_date, cardholder_name, cvv, salt, last_updated) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
            params![
                encrypted_title,
                encrypted_card_number,
                encrypted_expiry_date,
                encrypted_cardholder_name,
                encrypted_cvv,
                salt,
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
            ],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Add Credit Card: insertion failed: {}", e);
                return false;
            }
        };
        
        true
    }

    pub fn update_credit_card(&self, entry: CreditCardEntry) -> bool {
        let salt = encryption::generate_salt();
        let derived_key = match key_derivation::derive_key(&self.master_key, &salt) {
            Ok(derived_key) => derived_key,
            Err(e) => {
                eprintln!("Update Credit Card: key derivation failed: {}", e);
                return false;
            }
        };

        let encrypted_title = match encryption::encrypt_gcm(&entry.title, &derived_key)
        {
            Ok(encrypted_title) => encrypted_title,
            Err(e) => {
                eprintln!("Update Credit Card: title encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_card_number =
           match encryption::encrypt_gcm(&entry.card_number, &derived_key)
            {
                Ok(encrypted_card_number) => encrypted_card_number,
                Err(e) => {
                    eprintln!("Update Credit Card: card number encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_expiry_date =
            match encryption::encrypt_gcm(&entry.expiry_date, &derived_key)
            {
                Ok(encrypted_expiry_date) => encrypted_expiry_date,
                Err(e) => {
                    eprintln!("Update Credit Card: expiry date encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_cardholder_name =
            match encryption::encrypt_gcm(&entry.cardholder_name, &derived_key)
            {
                Ok(encrypted_cardholder_name) => encrypted_cardholder_name,
                Err(e) => {
                    eprintln!("Update Credit Card: cardholder name encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_cvv = match encryption::encrypt_gcm(&entry.cvv, &derived_key)
        {
            Ok(encrypted_cvv) => encrypted_cvv,
            Err(e) => {
                eprintln!("Update Credit Card: CVV encryption failed: {}", e);
                return false;
            } 
        };
        match self.conn.execute(
            "UPDATE CreditCardEntries 
             SET title = ?1, card_number = ?2, expiry_date = ?3, cardholder_name = ?4, cvv = ?5, salt = ?6, last_updated = ?7 
             WHERE id = ?8",
            params![
                encrypted_title,
                encrypted_card_number,
                encrypted_expiry_date,
                encrypted_cardholder_name,
                encrypted_cvv,
                salt,
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                entry.id
            ],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Update Credit Card: update failed: {}", e);
                return false;
            }
        };

        true
    }

    pub fn delete_credit_card(&self, id: &str) -> bool {
        match self.conn
            .execute("DELETE FROM CreditCardEntries WHERE id = ?1", params![id])
            {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Delete Credit Card: delete failed: {}", e);
                    return false;
            }
            };

        true
    }

    pub fn get_credit_cards(&self) -> Result<Vec<CreditCardEntry>, bool> {
        let mut stmt = match self.conn.prepare(
            "SELECT id, title, card_number, expiry_date, cardholder_name, cvv, salt, last_updated FROM CreditCardEntries",
        ){
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Get Credit Cards: connection statement failed: {}", e);
                return Err(false);
            } 
        };
        let entry_iter = match stmt.query_map([], |row| {
            let last_updated_datetime = row.get::<_, i64>(7).expect("Get Credit Cards: getting last update time failed");
            let datetime = DateTime::from_timestamp(last_updated_datetime, 0).unwrap();
            // Format as "day-month-year"
            let last_updated_datetime_string = datetime.format("%d.%m.%Y %H:%M:%S").to_string();

            let encoded_salt: String = row.get::<_, String>(6)?;
            let derived_key = match key_derivation::derive_key(&self.master_key, &encoded_salt) {
                Ok(derived_key) => derived_key,
                Err(_) => {
                    eprintln!("Get login: key derivation failed");
                    return Err(rusqlite::Error::InvalidQuery);
                }
            };
            
            let decrypted_title =
                match encryption::decrypt_gcm(&row.get::<_, String>(1).expect("Get Credit Cards: getting title failed"), &derived_key)
                {
                    Ok(decrypted_title) => decrypted_title,
                    Err(_) => {
                        eprintln!("Get Credit Cards: title decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
            let decrypted_card_number =
                match encryption::decrypt_gcm(&row.get::<_, String>(2).expect("Get Credit Cards: getting card number failed"), &derived_key)
                {
                    Ok(decrypted_card_number) => decrypted_card_number,
                    Err(_) => {
                        eprintln!("Get Credit Cards: card number decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
            let decrypted_expiry_date =
                match encryption::decrypt_gcm(&row.get::<_, String>(3).expect("Get Credit Cards: getting expiry date failed"), &derived_key)
                {
                    Ok(decrypted_expiry_date) => decrypted_expiry_date,
                    Err(_) => {
                        eprintln!("Get Credit Cards: expiry date decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
            let decrypted_cardholder_name =
                match encryption::decrypt_gcm(&row.get::<_, String>(4).expect("Get Credit Cards: getting cardholder name failed"), &derived_key)
                {
                    Ok(decrypted_cardholder_name) => decrypted_cardholder_name,
                    Err(_) => {
                        eprintln!("Get Credit Cards: cardholder name decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
            let decrypted_cvv =
                match encryption::decrypt_gcm(&row.get::<_, String>(5).expect("Get Credit Cards: getting CVV failed"), &derived_key)
                {
                    Ok(decrypted_cvv) => decrypted_cvv,
                    Err(_) => {
                        eprintln!("Get Credit Cards: CVV decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
                
            Ok(CreditCardEntry {
                id: row.get::<_, i64>(0)?.to_string(),
                title: decrypted_title,
                card_number: decrypted_card_number,
                expiry_date: decrypted_expiry_date,
                cardholder_name: decrypted_cardholder_name,
                cvv: decrypted_cvv,
                last_updated: last_updated_datetime_string,
            })
        }){
            Ok(entry_iter) => entry_iter,
            Err(e) => {
                eprintln!("Get Credit Cards: query failed: {}", e);
                return Err(false);
            } 
        };

        let mut entries = Vec::new();
        for entry in entry_iter {
            entries.push(entry.expect("Get Credit Cards: pushing entries error"));
        }
        Ok(entries)
    }

    pub fn add_note(&self, entry: NoteEntry) -> bool {
        let salt = encryption::generate_salt();
        let derived_key = match key_derivation::derive_key(&self.master_key, &salt) {
            Ok(derived_key) => derived_key,
            Err(e) => {
                eprintln!("Add Note: key derivation failed: {}", e);
                return false;
            }
        };

        let encrypted_title = match encryption::encrypt_gcm(&entry.title, &derived_key)
        {
            Ok(encrypted_title) => encrypted_title,
            Err(e) => {
                eprintln!("Add Note: title encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_content =
            match encryption::encrypt_gcm(&entry.content, &derived_key)
            {
                Ok(encrypted_content) => encrypted_content,
                Err(e) => {
                    eprintln!("Add Note: content encryption failed: {}", e);
                    return false;
                } 
            };

        match self.conn.execute(
            "INSERT INTO NoteEntries (title, salt, content, last_updated) VALUES (?1, ?2, ?3, ?4)",
            params![
                encrypted_title,
                salt,
                encrypted_content,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            ],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Add Note: insertion failed: {}", e);
                return false;
            }
        };

         true
    }

    pub fn update_note(&self, entry: NoteEntry) -> bool {
        let salt = encryption::generate_salt();
        let derived_key = match key_derivation::derive_key(&self.master_key, &salt) {
            Ok(derived_key) => derived_key,
            Err(e) => {
                eprintln!("Update Note: key derivation failed: {}", e);
                return false;
            }
        };

        let encrypted_title = match encryption::encrypt_gcm(&entry.title, &derived_key)
        {
            Ok(encrypted_title) => encrypted_title,
            Err(e) => {
                eprintln!("Update Note: title encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_content =
            match encryption::encrypt_gcm(&entry.content, &derived_key)
            {
                Ok(encrypted_content) => encrypted_content,
                Err(e) => {
                    eprintln!("Update Note: content encryption failed: {}", e);
                    return false;
                } 
            };
        match self.conn.execute(
            "UPDATE NoteEntries 
             SET title = ?1, salt = ?2, content = ?3, last_updated = ?4 
             WHERE id = ?5",
            params![
                encrypted_title,
                salt,
                encrypted_content,
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                entry.id
            ],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Update Note: update failed: {}", e);
                return false;
            }
        };
        
        true
    }

    pub fn delete_note(&self, id: &str) -> bool {
        match self.conn
            .execute("DELETE FROM NoteEntries WHERE id = ?1", params![id])
            {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Delete Note: delete failed: {}", e);
                    return false;
            }
            };

        true
    }

    pub fn get_notes(&self) -> Result<Vec<NoteEntry>, bool> {
        let mut stmt = match self
            .conn
            .prepare("SELECT id, title, salt, content, last_updated FROM NoteEntries")
            {
                Ok(stmt) => stmt,
                Err(e) => {
                    eprintln!("Get Notes: connection statement failed: {}", e);
                    return Err(false);
                } 
            };


        let entry_iter = match stmt.query_map([], |row| {
            let last_updated_datetime = row.get::<_, i64>(4).expect("Get Notes: getting last update time failed");
            let datetime = DateTime::from_timestamp(last_updated_datetime, 0).unwrap();
            // Format as "day-month-year"
            let last_updated_datetime_string = datetime.format("%d.%m.%Y %H:%M:%S").to_string();

            let encoded_salt: String = row.get::<_, String>(2).expect("Get Notes: getting salt failed");
            let derived_key = match key_derivation::derive_key(&self.master_key, &encoded_salt) {
                Ok(derived_key) => derived_key,
                Err(_) => {
                    eprintln!("Get Notes: key derivation failed");
                    return Err(rusqlite::Error::InvalidQuery);
                }
            };
            let decrypted_title =
                match encryption::decrypt_gcm(&row.get::<_, String>(1)?, &derived_key)
                {
                    Ok(decrypted_title) => decrypted_title,
                    Err(_) => {
                        eprintln!("Get Notes: title decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
            let decrypted_content =
                match encryption::decrypt_gcm(&row.get::<_, String>(3)?, &derived_key)
                {
                    Ok(decrypted_content) => decrypted_content,
                    Err(_) => {
                        eprintln!("Get Notes: content decryption failure");
                        return Err(rusqlite::Error::UnwindingPanic);
                    }
                };
            Ok(NoteEntry {
                id: row.get::<_, i64>(0)?.to_string(),
                title: decrypted_title,
                content: decrypted_content,
                last_updated: last_updated_datetime_string,
            })
        }){
            Ok(entry_iter) => entry_iter,
            Err(e) => {
                eprintln!("Get Logins: query failed: {}", e);
                return Err(false);
            } 
        };

        let mut entries = Vec::new();
        for entry in entry_iter {
            entries.push(entry.expect("Get Notes: pushing entries error"));
        }
        Ok(entries)
    }

    pub fn add_identity(&self, entry: IdentityEntry) -> bool {
        let salt = encryption::generate_salt();
        let derived_key = match key_derivation::derive_key(&self.master_key, &salt) {
            Ok(derived_key) => derived_key,
            Err(e) => {
                eprintln!("Add Identities: key derivation failed: {}", e);
                return false;
            }
        };
        let encrypted_title = match encryption::encrypt_gcm(&entry.title, &derived_key)
        {
            Ok(encrypted_title) => encrypted_title,
            Err(e) => {
                eprintln!("Add Identity: title encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_full_name =
            match encryption::encrypt_gcm(&entry.full_name, &derived_key)
            {
                Ok(encrypted_full_name) => encrypted_full_name,
                Err(e) => {
                    eprintln!("Add Identity: full name encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_date_of_birth =
            match encryption::encrypt_gcm(&entry.date_of_birth, &derived_key)
            {
                Ok(encrypted_date_of_birth) => encrypted_date_of_birth,
                Err(e) => {
                    eprintln!("Add Identity: date of birth encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_nationality =
            match encryption::encrypt_gcm(&entry.nationality, &derived_key)
            {
                Ok(encrypted_nationality) => encrypted_nationality,
                Err(e) => {
                    eprintln!("Add Identity: nationality encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_identification_number =
            match encryption::encrypt_gcm(&entry.identification_number, &derived_key)
            {
                Ok(encrypted_identification_number) => encrypted_identification_number,
                Err(e) => {
                    eprintln!("Add Identity: identification number encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_issue_date =
            match encryption::encrypt_gcm(&entry.issue_date, &derived_key)
            {
                Ok(encrypted_issue_date) => encrypted_issue_date,
                Err(e) => {
                    eprintln!("Add Identity: issue date encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_expiry_date =
            match encryption::encrypt_gcm(&entry.expiry_date, &derived_key)
            {
                Ok(encrypted_expiry_date) => encrypted_expiry_date,
                Err(e) => {
                    eprintln!("Add Identity: expiry date encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_issuer = match encryption::encrypt_gcm(&entry.issuer, &derived_key)
        {
            Ok(encrypted_issuer) => encrypted_issuer,
            Err(e) => {
                eprintln!("Add Identity: issuer encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_notes = match encryption::encrypt_gcm(&entry.notes, &derived_key)
        {
            Ok(encrypted_notes) => encrypted_notes,
            Err(e) => {
                eprintln!("Add Identity: note encryption failed: {}", e);
                return false;
            } 
        };

        match self.conn.execute(
            "INSERT INTO IdentityEntries (title, full_name, date_of_birth, nationality, identification_number, issue_date, expiry_date, issuer, notes, salt, last_updated)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11);",
            params![
                encrypted_title,
                encrypted_full_name,
                encrypted_date_of_birth,
                encrypted_nationality,
                encrypted_identification_number,
                encrypted_issue_date,
                encrypted_expiry_date,
                encrypted_issuer,
                encrypted_notes,
                salt,
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
            ],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Add Identity: insertion failed: {}", e);
                return false;
            }
        };

        true
    }

    pub fn update_identity(&self, entry: IdentityEntry) -> bool {
        let salt = encryption::generate_salt();
        let derived_key = match key_derivation::derive_key(&self.master_key, &salt) {
            Ok(derived_key) => derived_key,
            Err(e) => {
                eprintln!("Update Identities: key derivation failed: {}", e);
                return false;
            }
        };

        let encrypted_title = match encryption::encrypt_gcm(&entry.title, &derived_key)
        {
            Ok(encrypted_title) => encrypted_title,
            Err(e) => {
                eprintln!("Update Identity: title encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_full_name =
            match encryption::encrypt_gcm(&entry.full_name, &derived_key)
            {
                Ok(encrypted_full_name) => encrypted_full_name,
                Err(e) => {
                    eprintln!("Update Identity: full name encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_date_of_birth =
            match encryption::encrypt_gcm(&entry.date_of_birth, &derived_key)
            {
                Ok(encrypted_date_of_birth) => encrypted_date_of_birth,
                Err(e) => {
                    eprintln!("Update Identity: date of birth encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_nationality =
            match encryption::encrypt_gcm(&entry.nationality, &derived_key)
            {
                Ok(encrypted_nationality) => encrypted_nationality,
                Err(e) => {
                    eprintln!("Update Identity: nationality encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_identification_number =
            match encryption::encrypt_gcm(&entry.identification_number, &derived_key)
            {
                Ok(encrypted_identification_number) => encrypted_identification_number,
                Err(e) => {
                    eprintln!("Update Identity: identification number encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_issue_date =
            match encryption::encrypt_gcm(&entry.issue_date, &derived_key)
            {
                Ok(encrypted_issue_date) => encrypted_issue_date,
                Err(e) => {
                    eprintln!("Update Identity: issue date encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_expiry_date =
            match encryption::encrypt_gcm(&entry.expiry_date, &derived_key)
            {
                Ok(encrypted_expiry_date) => encrypted_expiry_date,
                Err(e) => {
                    eprintln!("Update Identity: expiry date encryption failed: {}", e);
                    return false;
                } 
            };
        let encrypted_issuer = match encryption::encrypt_gcm(&entry.issuer, &derived_key)
        {
            Ok(encrypted_issuer) => encrypted_issuer,
            Err(e) => {
                eprintln!("Update Identity: issuer encryption failed: {}", e);
                return false;
            } 
        };
        let encrypted_notes = match encryption::encrypt_gcm(&entry.notes, &derived_key)
        {
            Ok(encrypted_notes) => encrypted_notes,
            Err(e) => {
                eprintln!("Update Identity: note encryption failed: {}", e);
                return false;
            } 
        };

        match self.conn.execute(
            "UPDATE IdentityEntries 
                SET title = ?1, full_name = ?2, date_of_birth = ?3, nationality = ?4, identification_number = ?5, issue_date = ?6, 
                expiry_date = ?7, issuer = ?8, notes = ?9, salt = ?10, last_updated = ?11 
                WHERE id = ?12",
            params![
                encrypted_title,
                encrypted_full_name,
                encrypted_date_of_birth,
                encrypted_nationality,
                encrypted_identification_number,
                encrypted_issue_date,
                encrypted_expiry_date,
                encrypted_issuer,
                encrypted_notes,
                salt,
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
                entry.id
            ],
        ){
            Ok(_) => (),
            Err(e) => {
                eprintln!("Update Identity: update failed: {}", e);
                return false;
            }
        };
        
        true
    }

    pub fn delete_identity(&self, id: &str) -> bool {
        match self.conn
            .execute("DELETE FROM IdentityEntries WHERE id = ?1", params![id])
            {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Delete Identity: delete failed: {}", e);
                    return false;
                }
            };
        
        true
    }

    pub fn get_identities(&self) -> Result<Vec<IdentityEntry>, bool> {
        let mut stmt = match self.conn.prepare(
            "SELECT id, title, full_name, date_of_birth, nationality, identification_number,
                issue_date, expiry_date, issuer, notes, salt, last_updated 
                FROM IdentityEntries",
        )
        {
            Ok(stmt) => stmt,
            Err(e) => {
                eprintln!("Get Identities: connection statement failed: {}", e);
                return Err(false);
            } 
        };

        let entry_iter = match stmt.query_map([], |row| {
            let last_updated_datetime = row.get::<_, i64>(11).expect("Get Identities: getting last update time failed");
            let datetime = DateTime::from_timestamp(last_updated_datetime, 0).unwrap();
            // Format as "day-month-year"
            let last_updated_datetime_string = datetime.format("%d.%m.%Y %H:%M:%S").to_string();

            let encoded_salt: String = row.get::<_, String>(10).expect("Get Identities: getting salt failed");
            let derived_key = match key_derivation::derive_key(&self.master_key, &encoded_salt) {
                Ok(derived_key) => derived_key,
                Err(_) => {
                    eprintln!("Get Identities: key derivation failed");
                    return Err(rusqlite::Error::InvalidQuery);
                }
            };

            let decrypted_title =
                encryption::decrypt_gcm(&row.get::<_, String>(1).expect("Get Identities: getting title failed"), &derived_key).expect("Error");
            let decrypted_full_name =
                encryption::decrypt_gcm(&row.get::<_, String>(2).expect("Get Identities: getting full name failed"), &derived_key).expect("Error");
            let decrypted_date_of_birth =
                encryption::decrypt_gcm(&row.get::<_, String>(3).expect("Get Identities: getting date of birth failed"), &derived_key).expect("Error");
            let decrypted_nationality =
                encryption::decrypt_gcm(&row.get::<_, String>(4).expect("Get Identities: getting nationality failed"), &derived_key).expect("Error");
            let decrypted_identification_number =
                encryption::decrypt_gcm(&row.get::<_, String>(5).expect("Get Identities: getting identification number failed"), &derived_key).expect("Error");
            let decrypted_issue_date =
                encryption::decrypt_gcm(&row.get::<_, String>(6).expect("Get Identities: getting issue date failed"), &derived_key).expect("Error");
            let decrypted_expiry_date =
                encryption::decrypt_gcm(&row.get::<_, String>(7).expect("Get Identities: getting expiry date failed"), &derived_key).expect("Error");
            let decrypted_issuer =
                encryption::decrypt_gcm(&row.get::<_, String>(8).expect("Get Identities: getting issuer failed"), &derived_key).expect("Error");
            let decrypted_notes =
                encryption::decrypt_gcm(&row.get::<_, String>(9).expect("Get Identities: getting note failed"), &derived_key).expect("Error");

            Ok(IdentityEntry {
                id: row.get::<_, i64>(0)?.to_string(),
                title: decrypted_title,
                full_name: decrypted_full_name,
                date_of_birth: decrypted_date_of_birth,
                nationality: decrypted_nationality,
                identification_number: decrypted_identification_number,
                issue_date: decrypted_issue_date,
                expiry_date: decrypted_expiry_date,
                issuer: decrypted_issuer,
                notes: decrypted_notes,
                last_updated: last_updated_datetime_string,
            })
        })
        {
            Ok(entry_iter) => entry_iter,
            Err(e) => {
                eprintln!("Get Identities: query failed: {}", e);
                return Err(false);
            } 
        };

        let mut entries = Vec::new();
        for entry in entry_iter {
            entries.push(entry.expect("Get Identities: pushing entries error"));
        }
        Ok(entries)
    }
    
}