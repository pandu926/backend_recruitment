// src/routes/routes.rs
#[get("/")]
pub fn hello() -> &'static str {
    "Hello, wor!"
}

#[get("/ya")]
pub fn ok() -> &'static str {
    "Hello, ya"
}

pub fn create_routes() -> Vec<rocket::Route> {
    routes![hello, ok]
}
