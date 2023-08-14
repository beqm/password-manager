// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use argon2::password_hash::{rand_core::OsRng, PasswordHasher, SaltString};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use db::{create_client, create_item, del_item, edit_item, get_client, get_items, update_master_password};
use lazy_static::lazy_static;
use models::Items;
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

use chacha20poly1305::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    XChaCha20Poly1305,
};

use crate::db::toggle_app_lock;

mod db;
mod models;
mod schema;

lazy_static! {
    static ref USER: Mutex<String> = Mutex::new("NULL".to_string());
    static ref LOCK: Mutex<i32> = Mutex::new(0);
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TauriResponse<T> {
    data: T,
    status: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TauriClient {
    id: i32,
    username: String,
    items: Option<Vec<Items>>,
}

pub fn generate_recovery_code() -> String {
    use base32::Alphabet;
    use rand::Rng;
    use sha2::{Digest, Sha256};

    let alphabet = Alphabet::RFC4648 { padding: false };
    let random_bytes: Vec<u8> = (0..1000).map(|_| rand::thread_rng().gen::<u8>()).collect();
    let mut hasher = Sha256::new();
    hasher.update(random_bytes);
    let hash = hasher.finalize();

    base32::encode(alphabet, &hash)[0..=10].to_string()
}

#[tauri::command]
fn add_item(username: &str, title: &str, identify: &str, pass: &str, desc: &str, link: &str, _type: &str) -> String {
    let user = get_client(&username).unwrap();
    let mut stringified_pass = "".to_string();
    let mut stringified_desc = "".to_string();

    if pass.len() > 0 || desc.len() > 0 {
        let parsed_key = GenericArray::from_slice(user.master_password.as_bytes().get(..32).unwrap_or_default());
        let cipher = XChaCha20Poly1305::new(&parsed_key);
        let parsed_nonce = GenericArray::from_slice(user.recovery_code.as_bytes().get(..24).unwrap_or_default());

        if pass.len() > 0 {
            let encrypted_pass = cipher.encrypt(&parsed_nonce, pass.as_bytes().as_ref()).unwrap();
            stringified_pass = format!("{:?}", encrypted_pass);
        }

        if desc.len() > 0 {
            let encrypted_desc = cipher.encrypt(&parsed_nonce, desc.as_bytes().as_ref()).unwrap();
            stringified_desc = format!("{:?}", encrypted_desc);
        }
    }

    let result = create_item(user.id, title, identify, &stringified_pass, &stringified_desc, link, _type);
    match result {
        Some(i) => serde_json::to_string(&TauriResponse::<Items> { data: i, status: 200 }).unwrap(),
        None => serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap(),
    }
}

#[tauri::command]
fn update_item(username: &str, item_id: i32, title: &str, identify: &str, pass: &str, desc: &str, link: &str, _type: &str, created: i64) -> String {
    let user = get_client(&username).unwrap();
    let mut stringified_pass = "".to_string();
    let mut stringified_desc = "".to_string();

    if pass.len() > 0 || desc.len() > 0 {
        let parsed_key = GenericArray::from_slice(user.master_password.as_bytes().get(..32).unwrap_or_default());
        let cipher = XChaCha20Poly1305::new(&parsed_key);
        let parsed_nonce = GenericArray::from_slice(user.recovery_code.as_bytes().get(..24).unwrap_or_default());

        if pass.len() > 0 {
            let encrypted_pass = cipher.encrypt(&parsed_nonce, pass.as_bytes().as_ref()).unwrap();
            stringified_pass = format!("{:?}", encrypted_pass);
        }

        if desc.len() > 0 {
            let encrypted_desc = cipher.encrypt(&parsed_nonce, desc.as_bytes().as_ref()).unwrap();
            stringified_desc = format!("{:?}", encrypted_desc);
        }
    }

    let result = edit_item(user.id, item_id, title, identify, &stringified_pass, &stringified_desc, link, _type, created);
    match result {
        Some(i) => serde_json::to_string(&TauriResponse::<Items> { data: i, status: 200 }).unwrap(),
        None => serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap(),
    }
}

#[tauri::command]
fn remove_item(item_id: i32) -> String {
    let result = del_item(item_id);

    match result {
        Some(i) => serde_json::to_string(&TauriResponse::<Items> { data: i, status: 200 }).unwrap(),
        None => serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap(),
    }
}

#[tauri::command]
fn fetch_items(user_id: i32, username: &str) -> String {
    let result = get_items(&user_id);
    let user = get_client(&username).unwrap();

    match result {
        Some(mut v) => {
            for item in v.iter_mut() {
                let pass = item.password.as_ref().unwrap();
                let desc = item.description.as_ref().unwrap();
                if pass.len() > 0 || desc.len() > 0 {
                    let parsed_key = GenericArray::from_slice(user.master_password.as_bytes().get(..32).unwrap_or_default());
                    let cipher = XChaCha20Poly1305::new(&parsed_key);
                    let parsed_nonce = GenericArray::from_slice(user.recovery_code.as_bytes().get(..24).unwrap_or_default());

                    if pass.len() > 0 {
                        let serialized_pass: Vec<u8> = serde_json::from_str(&pass.replace('\'', "\"")).unwrap();
                        let decrypted_pass = cipher.decrypt(&parsed_nonce, serialized_pass.as_ref()).unwrap();
                        item.password = Some(String::from_utf8(decrypted_pass).unwrap());
                    }

                    if desc.len() > 0 {
                        let serialized_desc: Vec<u8> = serde_json::from_str(&desc.replace('\'', "\"")).unwrap();
                        let decrypted_desc = cipher.decrypt(&parsed_nonce, serialized_desc.as_ref()).unwrap();
                        item.description = Some(String::from_utf8(decrypted_desc).unwrap());
                    }
                }
            }

            serde_json::to_string(&TauriResponse::<Vec<Items>> { data: v, status: 200 }).unwrap()
        },
        None => serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap(),
    }
}

#[tauri::command]
fn register(username: &str, master_password: &str) -> String {
    let mut salt = SaltString::generate(&mut OsRng);
    let mut argon2 = Argon2::default();
    let password_hash = argon2.hash_password(master_password.as_bytes(), &salt).unwrap().to_string();

    let recovery_code = generate_recovery_code();
    salt = SaltString::generate(&mut OsRng);
    argon2 = Argon2::default();
    let recovery_hash = argon2.hash_password(recovery_code.as_bytes(), &salt).unwrap().to_string();

    let result = create_client(&username, &password_hash, &recovery_hash);

    match result {
        Some(_) => serde_json::to_string(&TauriResponse::<String> {
            data: recovery_code.to_string(),
            status: 200,
        })
        .unwrap(),
        None => serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap(),
    }
}

#[tauri::command]
fn login(username: &str, master_password: &str) -> String {
    let result = get_client(&username);
    match result {
        Ok(c) => {
            let parsed_hash = PasswordHash::new(&c.master_password).unwrap();
            let password_match = Argon2::default().verify_password(master_password.as_bytes(), &parsed_hash).is_ok();

            if password_match {
                *USER.lock().unwrap() = c.username.to_string();
                let items = get_items(&c.id);

                let data = TauriClient {
                    id: c.id,
                    username: c.username,
                    items,
                };

                serde_json::to_string(&TauriResponse {
                    data: Some(data),
                    status: 200,
                })
                .unwrap()
            } else {
                serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap()
            }
        },
        Err(_) => serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap(),
    }
}

#[tauri::command]
fn logout() -> String {
    *USER.lock().unwrap() = "NULL".to_string();
    serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 200 }).unwrap()
}

#[tauri::command]
fn verify_recovery_code(username: &str, recovery_code: &str) -> String {
    let result = get_client(&username);
    match result {
        Ok(c) => {
            let parsed_hash = PasswordHash::new(&c.recovery_code).unwrap();
            let recovery_match = Argon2::default().verify_password(recovery_code.as_bytes(), &parsed_hash).is_ok();

            if recovery_match {
                serde_json::to_string(&TauriResponse::<Option<String>> {
                    data: Some(c.username),
                    status: 200,
                })
                .unwrap()
            } else {
                serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap()
            }
        },
        Err(_) => serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap(),
    }
}

#[tauri::command]
fn change_password(username: &str, master_password: &str) -> String {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(master_password.as_bytes(), &salt).unwrap().to_string();
    let result = update_master_password(&username, &password_hash);

    match result {
        Ok(_) => serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 200 }).unwrap(),
        Err(_) => serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap(),
    }
}

#[tauri::command]
fn launch_website(url: String) -> () {
    let _ = open::that(url);
}

#[tauri::command]
fn generate_password(length: u32, upper: bool, numbers: bool, symbols: bool) -> String {
    let mut chars: Vec<char> = Vec::new();
    chars.extend('a'..='z');

    if upper {
        chars.extend('A'..='Z');
    }

    if numbers {
        chars.extend('0'..='9');
    }

    if symbols {
        chars.extend("!@#$%^&*()_+-=[]{}|;':,./<>?".chars());
    }

    let password: String = (0..length).map(|_| *chars.choose(&mut rand::thread_rng()).unwrap()).collect();

    password
}

#[tauri::command]
fn check_lock() -> bool {
    let user = get_client(&USER.lock().unwrap());

    if let Ok(c) = user {
        if c.app_lock == 1 {
            return true;
        } else {
            return false;
        }
    } else {
        return false;
    }
}

fn main() {
    println!("[INFO] APP INITIALIZED");
    let show = CustomMenuItem::new("open".to_string(), "Open");
    let lock = CustomMenuItem::new("lock".to_string(), "Lock");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(lock)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let system_tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::DoubleClick { position: _, size: _, .. } => {
                let window = app.get_window("main").unwrap();
                window.show().unwrap();
                window.set_focus().unwrap();
            },
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "open" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                },
                "lock" => {
                    let user = get_client(&USER.lock().unwrap());
                    if let Ok(c) = user {
                        toggle_app_lock(c).unwrap();
                    }
                },
                "quit" => {
                    std::process::exit(0);
                },
                _ => {},
            },
            _ => {
                let user = get_client(&USER.lock().unwrap());
                if let Ok(c) = user {
                    let item_handle = app.tray_handle().get_item("lock");
                    if c.app_lock == 1 {
                        item_handle.set_title("Lock \u{2714}").unwrap();
                    } else {
                        item_handle.set_title("Lock").unwrap();
                    }
                }
            },
        })
        .invoke_handler(tauri::generate_handler![
            launch_website, generate_password, register, login, logout, verify_recovery_code, change_password, add_item, fetch_items, update_item,
            remove_item, check_lock
        ])
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| match event {
            tauri::RunEvent::WindowEvent { label, event: win_event, .. } => match win_event {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    let win = app.get_window(label.as_str()).unwrap();
                    win.hide().unwrap();
                    api.prevent_close();
                },
                _ => {},
            },
            _ => {},
        })
}
