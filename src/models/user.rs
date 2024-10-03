use crate::schema::users;
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,              // Field untuk role dengan default 'USER'
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String, // Role dapat diatur saat insert
}

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub email: String,
    pub password: String,
    
}
