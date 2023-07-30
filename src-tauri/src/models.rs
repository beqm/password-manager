use crate::schema::client;
use diesel::prelude::*;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = client)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Client {
    pub id: i32,
    pub username: String,
    pub master_password: String,
    pub recovery_code: String,
}

#[derive(Insertable)]
#[diesel(table_name = client)]
pub struct NewClient<'a> {
    pub username: &'a str,
    pub master_password: &'a str,
    pub recovery_code: &'a str,
}
