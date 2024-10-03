use diesel::prelude::*;
use crate::models::user::{User, NewUser};
use crate::db::establish_connection;
use crate::schema::users;

pub fn send( name: String, email: String, password: String, role:String) -> User {
    
    let mut conn = establish_connection();
    let new_post = NewUser { name, email, password, role };

    diesel::insert_into(users::table)
        .values(&new_post)
        .get_result(&mut conn)
        .expect("Error saving new post")
}

pub fn get() -> Vec<User> {
    let mut conn: PgConnection = establish_connection();
    users::table
        .load(&mut conn)
        .expect("Error retrieving users")
}

pub fn delete(id: i32) -> Result<(), diesel::result::Error> {
    let mut conn = establish_connection();
    diesel::delete(users::table.find(id))
        .execute(&mut conn)?;
    Ok(())
}

pub fn edit(id: i32, name: String, email: String, password: String) -> Result<(), diesel::result::Error> {
    let mut conn = establish_connection();
    diesel::update(users::table.find(id))
        .set((
            users::name.eq(name),
            users::email.eq(email),
            users::password.eq(password),
        )) // Specify fields to update
        .execute(&mut conn)?;
    Ok(())
}

pub fn get_single(email: String) -> Vec<User> {
    let mut conn: PgConnection = establish_connection();
    users::table
        .filter(users::email.eq(email))
        .load(&mut conn)
        .expect("Error retrieving users")
}