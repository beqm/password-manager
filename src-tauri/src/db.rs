use crate::models::Client;
use crate::models::NewClient;
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_client(conn: &mut SqliteConnection, username: &str, master_password: &str) -> Option<Client> {
    use crate::schema::client;
    let recovery_code = generate_recovery_code();

    let new_client = NewClient {
        username,
        master_password,
        recovery_code: &recovery_code,
    };

    diesel::insert_or_ignore_into(client::table)
        .values(&new_client)
        .returning(Client::as_returning())
        .get_result(conn)
        .optional()
        .expect("Error creating new client")
}

#[derive(Debug)]
pub enum ClientError {
    ClientNotFound,
}
pub fn get_client(conn: &mut SqliteConnection, user: &str) -> Result<Client, ClientError> {
    use crate::schema::client::dsl::*;

    let result = client.filter(username.eq(user)).select(Client::as_select()).first(conn);

    match result {
        Ok(c) => return Ok(c),
        Err(_) => return Err(ClientError::ClientNotFound),
    }
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
