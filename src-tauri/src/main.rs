// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use argon2::{Argon2, PasswordHash, PasswordVerifier};
use db::{create_client, get_client, update_master_password};
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

mod db;
mod models;
mod schema;

#[derive(Serialize, Deserialize, Debug)]
pub struct TauriResponse<T> {
    data: T,
    status: u32,
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

    base32::encode(alphabet, &hash)
}

#[tauri::command]
fn register(username: &str, master_password: &str) -> String {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

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
                serde_json::to_string(&TauriResponse { data: Some(c), status: 200 }).unwrap()
            } else {
                serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap()
            }
        },
        Err(_) => serde_json::to_string(&TauriResponse::<Option<String>> { data: None, status: 400 }).unwrap(),
    }
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

fn main() {
    // Client::initialize_table();
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let show = CustomMenuItem::new("open".to_string(), "Open");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
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
                "quit" => {
                    std::process::exit(0);
                },
                "open" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                    window.set_focus().unwrap();
                },
                _ => {},
            },
            _ => {},
        })
        .invoke_handler(tauri::generate_handler![launch_website, generate_password, register, login, verify_recovery_code, change_password])
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
