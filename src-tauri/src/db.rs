use crate::models::Client;
use crate::models::Items;
use crate::models::NewClient;
use crate::models::NewItem;
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::env;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    conn.run_pending_migrations(MIGRATIONS).unwrap();
    conn
}

pub fn create_client(username: &str, master_password: &str, recovery_code: &str) -> Option<Client> {
    use crate::schema::client;

    let mut conn = establish_connection();

    let new_client = NewClient {
        username,
        master_password,
        recovery_code,
    };

    diesel::insert_or_ignore_into(client::table)
        .values(&new_client)
        .returning(Client::as_returning())
        .get_result(&mut conn)
        .optional()
        .expect("Error creating new client")
}

#[derive(Debug)]
pub enum ClientError {
    ClientNotFound,
}

pub fn get_client(user: &str) -> Result<Client, ClientError> {
    use crate::schema::client::dsl::*;

    let mut conn = establish_connection();

    let result = client.filter(username.eq(user)).select(Client::as_select()).first(&mut conn);

    match result {
        Ok(c) => return Ok(c),
        Err(_) => return Err(ClientError::ClientNotFound),
    }
}

pub fn update_master_password(user: &str, password: &str) -> Result<Client, ClientError> {
    use crate::schema::client::dsl::*;

    let mut conn = establish_connection();

    let result = diesel::update(client.filter(username.eq(user)))
        .set(master_password.eq(password))
        .get_result(&mut conn);

    match result {
        Ok(c) => return Ok(c),
        Err(_) => return Err(ClientError::ClientNotFound),
    }
}

pub fn create_item(user_id: i32, title: &str, identify: &str, pass: &str, desc: &str, link: &str, _type: &str) -> Option<Items> {
    use crate::schema::items;
    use chrono::Local;

    let new_item = NewItem {
        title,
        identifier: identify,
        password: pass,
        description: desc,
        link,
        type_: _type,
        client_id: user_id,
        created_at: Local::now().timestamp_millis(),
        last_modified: Local::now().timestamp_millis(),
    };

    let mut conn = establish_connection();

    diesel::insert_or_ignore_into(items::table)
        .values(&new_item)
        .returning(Items::as_returning())
        .get_result(&mut conn)
        .optional()
        .expect("Error creating new Item")
}

pub fn edit_item(
    user_id: i32, item_id: i32, ititle: &str, identify: &str, pass: &str, desc: &str, ilink: &str, _type: &str, created: i64,
) -> Option<Items> {
    use crate::schema::items::dsl::*;
    use chrono::Local;

    let item = NewItem {
        title: ititle,
        identifier: identify,
        password: pass,
        description: desc,
        link: ilink,
        type_: _type,
        client_id: user_id,
        created_at: created,
        last_modified: Local::now().timestamp_millis(),
    };

    let mut conn = establish_connection();

    let result = diesel::update(items.filter(id.eq(item_id))).set(item).get_result(&mut conn);

    match result {
        Ok(c) => return Some(c),
        Err(_) => return None,
    }
}

pub fn del_item(item_id: i32) -> Option<Items> {
    use crate::schema::items::dsl::*;

    let mut conn = establish_connection();

    let result = diesel::delete(items.filter(id.eq(item_id))).get_result(&mut conn);

    match result {
        Ok(c) => return Some(c),
        Err(_) => return None,
    }
}

pub fn get_items(user_id: &i32) -> Option<Vec<Items>> {
    use crate::schema::items::dsl::*;
    let mut conn = establish_connection();

    let result = items.filter(client_id.eq(user_id)).select(Items::as_select()).get_results(&mut conn);

    match result {
        Ok(c) => return Some(c),
        Err(_) => return None,
    }
}
