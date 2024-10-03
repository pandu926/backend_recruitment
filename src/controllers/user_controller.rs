use rocket::serde::json::Json;
use rocket::http::Status;
use crate::models::user::{User,NewUser, Login};
use crate::services::user_service;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Serialize, Deserialize};
use crate::middleware::jwt_authorization::JwtToken;
use std::env;
use bcrypt::{hash, DEFAULT_COST, verify};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Serialize)]
pub struct ResponeTemplate<T> {
    pub msg: String,
    pub status: i32,
    pub data: T,
}

#[post("/user", format = "json", data = "<new_post>")]
pub fn create_post(new_post: Json<NewUser>) -> Json<User> {
    let data_user = new_post.into_inner();
    let hashed_password = hash(data_user.password, DEFAULT_COST).unwrap();
    let user = user_service::send(data_user.name, data_user.email, hashed_password, data_user.role);
    Json(user)
}

#[get("/user/list")]
pub fn get_data(token: JwtToken) -> Result<Json<ResponeTemplate<Vec<User>>>, Status> {
    if token.0.role == "admin" {
        let data = user_service::get();
        let response = ResponeTemplate {
            msg: "Success".to_string(),
            status: 200,
            data: data,
        }; // Fetch data from the service
        Ok(Json(response)) // Return the data wrapped in Json
    } else {
        Err(Status::Forbidden) // Return Forbidden if not an admin
    }
}

#[delete("/user/<id>")]
pub fn delete_user(id: i32) -> Result<String, rocket::http::Status> {
    user_service::delete(id).map(|_| "sukses hapus".to_string())
        .map_err(|_| rocket::http::Status::NotFound)
}

#[patch("/user/<id>", format = "json", data = "<updated_user>")]
pub fn edit_user(id: i32, updated_user: Json<NewUser>, token: JwtToken) -> Result<Json<String>, Status> {
    let updated_data = updated_user.into_inner();
    let hashed_password = hash(updated_data.password, DEFAULT_COST).unwrap();
    // Check if the user is an admin or the owner of the user ID
    if token.0.role == "admin" || token.0.id == id {
        // Perform the update operation
        match user_service::edit(id, updated_data.name, updated_data.email, hashed_password) {
            Ok(_) => Ok(Json("User updated successfully".to_string())),
            Err(_) => Err(Status::NotFound),
        }
    } else {
        Err(Status::Forbidden) // Return Forbidden if the user is neither an admin nor the owner
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub username: String,
    pub exp: usize,
    pub role: String,
}

#[post("/user/login", format = "json", data = "<credentials>")]
pub async fn login(credentials: Json<Login>) -> Result<Json<AuthResponse>, Status> {
    let credentials = credentials.into_inner();
    let email = credentials.email;
    let password = credentials.password;

    // Fetch user data from the database
    let user = user_service::get_single(email.clone()).into_iter().next();

    match user {
        Some(user) => {
            // Verify the password
            if verify(password, &user.password).unwrap_or(false) {
                // Generate JWT token
                let claims = Claims {
                    id: user.id,
                    username: user.name,
                    exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
                    role: user.role,
                };
                let secret = env::var("JWT_TOKEN").expect("JWT_TOKEN must be set");
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
                    .map_err(|_| Status::InternalServerError)?;

                Ok(Json(AuthResponse { token }))
            } else {
                Err(Status::Unauthorized)
            }
        }
        None => Err(Status::Unauthorized),
    }
}
