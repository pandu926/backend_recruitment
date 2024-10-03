// use rocket::serde::json::Json;
// use crate::models::admin::{Admin, NewAdmin};
// use crate::services::admin_service;
// use dotenvy::dotenv;
// use serde::{Deserialize, Serialize};
// use jsonwebtoken::{encode, Header, EncodingKey, get_current_timestamp};
// use std::env;


// #[derive(Debug, Serialize, Deserialize)]
// struct AdminClaims {
//     id: i32,
//     username: String,
//     exp: usize
// }


// fn generate_jwt(admin: Admin) -> String {
//     dotenv().ok();
//     let secret_key = env::var("JWT_TOKEN").expect("JWT_TOKEN must be set");
//     let expiration = get_current_timestamp() + 3600;
//     let claims = AdminClaims {
//         id: admin.id,
//         username: admin.username,
//         exp: expiration as usize
//     };
//     let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref())).expect("Unable to encode token");
//     token
// }

// #[post("/auth/login", format = "json", data = "<credentials>")]
// pub fn login(credentials: Json<NewAdmin>) -> Result<Json<String>, rocket::http::Status> {
//     let credentials = credentials.into_inner();
//     admin_service::login(credentials.username, credentials.password)
//         .map_err(|_| rocket::http::Status::Unauthorized)
//         .map(|admin| {
//             let token = generate_jwt(admin);
//             Json(token)
//         })
// }



