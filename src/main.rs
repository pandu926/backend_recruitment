// src/main.rs
#[macro_use] extern crate rocket;

mod routes;
mod controllers;
mod models;
mod schema;
mod services;
mod db;
mod middleware;
use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok(); 
    rocket::build().mount("/", routes::create_routes())
}