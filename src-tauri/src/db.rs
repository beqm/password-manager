use crate::models::Client;
use crate::models::NewClient;
use diesel::prelude::*;
use diesel::update;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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
