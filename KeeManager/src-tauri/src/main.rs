// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod config;
mod database;
mod encode;
mod encryption;
mod entry;
mod hash;
mod key_derivation;

use entry::IdentityEntry;
use entry::NoteEntry;
use tauri::WindowEvent;
use crate::config::Config;
use crate::database::Database;
use crate::entry::CreditCardEntry;
use crate::entry::LoginEntry;
use std::str;

#[tauri::command]
fn add_login(db_path: &str, master_key: &str, entry: entry::LoginEntry) -> bool {
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );
    // Add an entry
    {
        let db = db_instance.lock().unwrap();

        db.add_login(entry)
    }

}

#[tauri::command]
fn add_credit_card(db_path: &str, master_key: &str, entry: entry::CreditCardEntry) -> bool {
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );
    // Add an entry
    {
        let db = db_instance.lock().unwrap();

        db.add_credit_card(entry)
    }
}

#[tauri::command]
fn add_note(db_path: &str, master_key: &str, entry: entry::NoteEntry) -> bool {
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );
    // Add an entry
    {
        let db = db_instance.lock().unwrap();

        db.add_note(entry)
    }
}

#[tauri::command]
fn add_identity(db_path: &str, master_key: &str, entry: entry::IdentityEntry) -> bool {
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );
    // Add an entry
    {
        let db = db_instance.lock().unwrap();

        db.add_identity(entry)
    }
}

#[tauri::command]
fn get_logins(db_path: &str, master_key: &str) -> Result<Vec<LoginEntry>, String> {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    let entries: Vec<LoginEntry>;

    {
        // Lock the database instance and get entries
        let db = db_instance.lock().unwrap();

        entries = db.get_logins().map_err(|e| e.to_string())?;
        // Optionally print entries for debugging
        println!("\n-------------------------------------------");
        for entry in &entries {
            println!("ID: {}", entry.id);
            println!("Title: {}", entry.title);
            println!("Username: {}", entry.username);
            println!("Password Hash: {}", entry.password);
            println!("Url: {}", entry.url);
            println!("Notes: {}", entry.notes);
            println!("Last Updated: {}", entry.last_updated);
            println!("-------------------------------------------");
        }
    }
    // Return the entries
    Ok(entries)
}

#[tauri::command]
fn get_credit_cards(db_path: &str, master_key: &str) -> Result<Vec<CreditCardEntry>, String> {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    let entries: Vec<CreditCardEntry>;

    {
        // Lock the database instance and get entries
        let db = db_instance.lock().unwrap();

        entries = db.get_credit_cards().map_err(|e| e.to_string())?;
    }
    // Return the entries
    Ok(entries)
}

#[tauri::command]
fn get_notes(db_path: &str, master_key: &str) -> Result<Vec<NoteEntry>, String> {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    let entries: Vec<NoteEntry>;

    {
        // Lock the database instance and get entries
        let db = db_instance.lock().unwrap();

        entries = db.get_notes().map_err(|e| e.to_string())?;
    }
    // Return the entries
    Ok(entries)
}

#[tauri::command]
fn get_identities(db_path: &str, master_key: &str) -> Result<Vec<IdentityEntry>, String> {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    let entries: Vec<IdentityEntry>;

    {
        // Lock the database instance and get entries
        let db = db_instance.lock().unwrap();

        entries = db.get_identities().map_err(|e| e.to_string())?;
    }
    // Return the entries
    Ok(entries)
}

#[tauri::command]
fn delete_login(db_path: &str, master_key: &str, id: &str) -> () {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    {
        // Lock the database instance and get entries
        let db = db_instance.lock().unwrap();
        let _ = db.delete_login(id);
    }
}

#[tauri::command]
fn delete_credit_card(db_path: &str, master_key: &str, id: &str) -> () {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    {
        // Lock the database instance and get entries
        let db = db_instance.lock().unwrap();
        let _ = db.delete_credit_card(id);
    }
}

#[tauri::command]
fn delete_note(db_path: &str, master_key: &str, id: &str) -> () {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    {
        // Lock the database instance and get entries
        let db = db_instance.lock().unwrap();
        let _ = db.delete_note(id);
    }
}

#[tauri::command]
fn delete_identity(db_path: &str, master_key: &str, id: &str) -> () {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    {
        // Lock the database instance and get entries
        let db = db_instance.lock().unwrap();
        let _ = db.delete_identity(id);
    }
}

#[tauri::command]
fn update_login(db_path: &str, master_key: &str, entry: entry::LoginEntry) -> () {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    {
        let db = db_instance.lock().unwrap();
        let _ = db.update_login(entry);
    }
}

#[tauri::command]
fn update_credit_card(db_path: &str, master_key: &str, entry: entry::CreditCardEntry) -> () {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    {
        let db = db_instance.lock().unwrap();
        let _ = db.update_credit_card(entry);
    }
}

#[tauri::command]
fn update_note(db_path: &str, master_key: &str, entry: entry::NoteEntry) -> () {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    {
        let db = db_instance.lock().unwrap();
        let _ = db.update_note(entry);
    }
}

#[tauri::command]
fn update_identity(db_path: &str, master_key: &str, entry: entry::IdentityEntry) -> () {
    // Get the database instance
    let db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    {
        let db = db_instance.lock().unwrap();
        let _ = db.update_identity(entry);
    }
}

#[tauri::command]
fn get_database_path() -> Result<String, String> {
    match Config::load() {
        Ok(config) => Ok(config.database_path),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn set_database_path(db_path: &str) -> Result<(), String> {
    let mut config = match Config::load() {
        Ok(config) => config,
        Err(e) => return Err(e.to_string()),
    };
    config.database_path = db_path.to_string();
    match config.save() {
        Ok(()) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn is_database_valid(db_path: &str) -> bool {
    Database::database_exists(db_path)
}

#[tauri::command]
fn save_master_key_hash(db_path: &str, master_key: &str) -> bool {
    Database::save_master_key_hash(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    )
}

#[tauri::command]
fn check_master_key_hash(db_path: &str, master_key: &str) -> bool {
    Database::check_master_key_hash(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    )
}

#[tauri::command]
fn create_database(db_path: &str, master_key: &str) -> bool {
    if Database::database_exists(db_path) {
        return false;
    }

    let _db_instance = database::Database::get_instance(
        db_path,
        &encode::encode_base64(master_key.as_bytes().to_vec()),
    );

    Database::database_exists(db_path)
}


#[tauri::command]
fn minimize_window(window: tauri::Window) {
    window.minimize().unwrap();
}

#[tauri::command]
fn maximize_window(window: tauri::Window) {
    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}

#[tauri::command]
fn close_window(window: tauri::Window) {
    window.close().unwrap();
}

#[tauri::command]
fn logout() {
    database::Database::clear_instance();
}

fn main() {
    tauri::Builder::default()
        .on_window_event(|_app_handle, event| {
            if let WindowEvent::CloseRequested { .. } = event {
                database::Database::clear_instance();
            // If you want to prevent the window from closing, you can:
            // api.prevent_close();
            }
        })
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            create_database,
            add_login,
            add_credit_card,
            add_note,
            add_identity,
            get_logins,
            get_credit_cards,
            get_notes,
            get_identities,
            get_database_path,
            set_database_path,
            is_database_valid,
            update_login,
            update_credit_card,
            update_note,
            update_identity,
            delete_login,
            delete_credit_card,
            delete_note,
            delete_identity,
            check_master_key_hash,
            save_master_key_hash,
            minimize_window,
            maximize_window,
            close_window,
            logout
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
