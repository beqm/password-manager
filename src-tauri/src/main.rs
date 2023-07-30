// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fmt::format;

use db::{create_client, establish_connection};
use rand::prelude::SliceRandom;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

mod db;
mod models;
mod schema;

#[tauri::command]
fn register(username: &str, master_password: &str) -> String {
    let mut conn = establish_connection();

    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(master_password.as_bytes(), &salt).unwrap().to_string();

    let result = create_client(&mut conn, &username, &password_hash);
    match result {
        Some(c) => return format!("{}", c.recovery_code),
        None => return format!("Username already taken!",),
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
        .invoke_handler(tauri::generate_handler![launch_website, generate_password, register])
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
