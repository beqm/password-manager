use crate::schema::client;
use crate::schema::items;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = client)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Client {
    pub id: i32,
    pub username: String,
    pub master_password: String,
    pub recovery_code: String,
    pub app_lock: i32,
    pub min_tray: i32,
}

#[derive(Insertable)]
#[diesel(table_name = client)]
pub struct NewClient<'a> {
    pub username: &'a str,
    pub master_password: &'a str,
    pub recovery_code: &'a str,
    pub app_lock: i32,
    pub min_tray: i32,
}

#[derive(Debug, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Items {
    pub id: i32,
    pub title: String,
    pub identifier: Option<String>,
    pub password: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub type_: String,
    pub created_at: i64,
    pub last_modified: i64,
    pub client_id: i32,
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = items)]
pub struct NewItem<'a> {
    pub title: &'a str,
    pub identifier: &'a str,
    pub password: &'a str,
    pub description: &'a str,
    pub link: &'a str,
    pub type_: &'a str,
    pub created_at: i64,
    pub last_modified: i64,
    pub client_id: i32,
}
