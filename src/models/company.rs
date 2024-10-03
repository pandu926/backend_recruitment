use crate::schema::companies;
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = companies)]
pub struct Company {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub owner_id:  Option<i32>, // Referensi ke user sebagai pemilik
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = companies)]
pub struct NewCompany {
    pub name: String,
    pub description: String,
    pub owner_id: i32, // Harus diberikan saat insert
}
