use crate::schema::job_applications;
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = job_applications)]
pub struct JobApplication {
    pub id: i32,
    pub user_id: i32,        // Referensi ke user (pelamar)
    pub job_id: i32,         // Referensi ke job yang dilamar
    pub status: String,      // Status aplikasi (PENDING, ACCEPTED, REJECTED)
    pub applied_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = job_applications)]
pub struct NewJobApplication {
    pub user_id: i32, // User yang melamar
    pub job_id: i32,  // Job yang dilamar
    pub status: String, // Status awal aplikasi, misalnya "PENDING"
}
