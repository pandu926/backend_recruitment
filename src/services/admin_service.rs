// use diesel::prelude::*;
// use crate::models::admin::Admin;
// use crate::db::establish_connection;
// use crate::schema::admins;

// pub fn login(username: String, password: String) -> Result<Admin, diesel::result::Error> {
//     let mut conn = establish_connection();
//     admins::table
//         .filter(admins::username.eq(username))
//         .filter(admins::password.eq(password))
//         .get_result(&mut conn)
// }
