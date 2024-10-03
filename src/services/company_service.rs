use diesel::prelude::*;
use crate::models::company::{Company, NewCompany};
use crate::schema::companies::dsl::*;
use crate::db::establish_connection;
use crate::schema::companies; // Pastikan ini menginisialisasi koneksi

pub fn add(data_company: NewCompany) -> Company {
    let mut conn = establish_connection(); // Inisialisasi koneksi ke database

    // Menyimpan data ke dalam database
    diesel::insert_into(companies)
        .values(&data_company)
        .get_result(&mut conn) // Pastikan result-nya bertipe Company
        .expect("Error saving new company") // Berikan pesan error yang sesuai
}

pub fn get_all() -> Vec<Company> {
    let mut conn: PgConnection = establish_connection();
    companies::table
        .load(&mut conn)
        .expect("Error retrieving users")
}

pub fn delete(id: i32)-> Result<(), diesel::result::Error> {
    let mut conn = establish_connection();
    diesel::delete(companies::table.find(id))
        .execute(&mut conn)?;
    Ok(())
}

pub fn update(company: NewCompany, id: i32)-> Vec<Company> {
    
}