use crate::schema::jobs;
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use bigdecimal::BigDecimal;
use std::time::SystemTime;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = jobs)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub requirements: String,
    pub salary: BigDecimal, // Use Decimal for precise decimal calculations
    pub location: String,
    pub company_id: i32, // Relation to the Company table
    pub created_by_id: i32, // Relation to the user who created the job
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

#[derive(Insertable, Deserialize, Serialize)]
#[diesel(table_name = jobs)]
pub struct NewJob {
    pub title: String,
    pub description: String,
    pub requirements: String,
    pub salary: BigDecimal, // Use Decimal for precise decimal calculations
    pub location: String,
    pub company_id: i32, // Reference to the company
    pub created_by_id: i32, // Reference to the user who created the job
}

// If necessary, add a function to convert f64 to Decimal:
