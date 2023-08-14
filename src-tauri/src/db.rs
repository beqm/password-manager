use crate::models::Client;
use crate::models::Items;
use crate::models::NewClient;
use crate::models::NewItem;
use diesel::prelude::*;
use diesel::SqliteConnection;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

pub fn establish_connection() -> SqliteConnection {
    println!("[INFO] Connecting to database");

    // Build url
    // let database_url = "_up_/data/data.db";

    // Dev url
    let database_url = "../data/data.db";
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
        app_lock: 0,
    };

    let result = diesel::insert_or_ignore_into(client::table)
        .values(&new_client)
        .returning(Client::as_returning())
        .get_result(&mut conn)
        .optional();

    match result {
        Ok(c) => {
            println!("[SUCCESS] Creating new client: {:?}", &username);
            return c;
        },
        Err(err) => {
            println!("[ERROR] Failed Creating new client: {:?}", err);
            return None;
        },
    }
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
        Ok(c) => {
            println!("[SUCCESS] retrieving data of: {:?}", &user);
            return Ok(c);
        },
        Err(_) => {
            println!("[ERROR] user doesn't exist: {:?}", &user);
            return Err(ClientError::ClientNotFound);
        },
    }
}

pub fn update_master_password(user: &str, password: &str) -> Result<Client, ClientError> {
    use crate::schema::client::dsl::*;

    let mut conn = establish_connection();
    let result = diesel::update(client.filter(username.eq(user)))
        .set(master_password.eq(password))
        .get_result(&mut conn);

    match result {
        Ok(c) => {
            println!("[SUCCESS] updating data of: {:?}", &user);
            return Ok(c);
        },
        Err(_) => {
            println!("[ERROR] user error not found: {:?}", &user);
            return Err(ClientError::ClientNotFound);
        },
    }
}

pub fn toggle_app_lock(user: Client) -> Result<Client, ClientError> {
    use crate::schema::client::dsl::*;
    let inverted_value: i32;

    if user.app_lock == 0 {
        inverted_value = 1
    } else {
        inverted_value = 0
    }

    let mut conn = establish_connection();
    let result = diesel::update(client.filter(username.eq(&user.username)))
        .set(app_lock.eq(inverted_value))
        .get_result(&mut conn);

    match result {
        Ok(c) => {
            println!("[SUCCESS] setting app of: {:?} to: {:?}", &user.username, &inverted_value);
            return Ok(c);
        },
        Err(_) => {
            println!("[ERROR] something went wrong: {:?}", &user.username);
            return Err(ClientError::ClientNotFound);
        },
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
    let result = diesel::insert_or_ignore_into(items::table)
        .values(&new_item)
        .returning(Items::as_returning())
        .get_result(&mut conn)
        .optional();

    match result {
        Ok(c) => {
            println!("[SUCCESS] Creating item: {:?}, TYPE: {:?}", &title, &_type);
            return c;
        },
        Err(_) => {
            println!("[ERROR] Error Creating item: {:?}, TYPE: {:?}", &title, &_type);
            return None;
        },
    }
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
        Ok(c) => {
            println!("[SUCCESS] Editting item: {:?}, TYPE: {:?}", &ititle, &_type);
            return Some(c);
        },
        Err(_) => {
            println!("[ERROR] Error Occured: {:?}, TYPE: {:?}", &ititle, &_type);
            return None;
        },
    }
}

pub fn del_item(item_id: i32) -> Option<Items> {
    use crate::schema::items::dsl::*;
    let mut conn = establish_connection();
    let result = diesel::delete(items.filter(id.eq(item_id))).get_result(&mut conn);

    match result {
        Ok(c) => {
            println!("[SUCCESS] Deleting item: {:?}", item_id);
            return Some(c);
        },
        Err(_) => {
            println!("[ERROR] Item not found: {:?}", item_id);
            return None;
        },
    }
}

pub fn get_items(user_id: &i32) -> Option<Vec<Items>> {
    use crate::schema::items::dsl::*;
    let mut conn = establish_connection();
    let result = items.filter(client_id.eq(user_id)).select(Items::as_select()).get_results(&mut conn);

    match result {
        Ok(c) => {
            println!("[SUCCESS] Fetching items from: {:?}", &user_id);
            return Some(c);
        },
        Err(_) => {
            println!("[ERROR] Items not found: {:?}", &user_id);
            return None;
        },
    }
}
