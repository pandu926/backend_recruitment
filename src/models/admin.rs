// use crate::schema::admins;
// use diesel::{Queryable, Insertable};
// use serde::{Deserialize, Serialize};
// use std::time::SystemTime;

// #[derive(Queryable, Insertable, Serialize, Deserialize)]
// #[diesel(table_name = admins)]
// pub struct Admin {
//     pub id: i32,
//     pub username: String,
//     pub password: String,
//     pub created_at: SystemTime,
//     pub updated_at: SystemTime,
// }

// #[derive(Insertable, Deserialize, Serialize)]
// #[diesel(table_name = admins)]
// pub struct NewAdmin {
//     pub username: String,
//     pub password: String,
// }