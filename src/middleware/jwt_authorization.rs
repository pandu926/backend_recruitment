use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm, TokenData, errors::Error};
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: i32,
    pub username: String,
    pub exp: usize,
    pub role: String,
}

pub struct JwtToken(pub Claims);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JwtToken {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        dotenv().ok();
        let secret = env::var("JWT_TOKEN").expect("JWT_TOKEN must be set");
        let auth_header = req.headers().get_one("Authorization");
        
        match auth_header {
            Some(token) if token.starts_with("Bearer ") => {
                let token = token.trim_start_matches("Bearer ");
                match decode_token(token, &secret) {
                    Ok(token_data) => Outcome::Success(JwtToken(token_data.claims)),
                    Err(err) => {
                        println!("Error decoding token: {:?}", err);
                        Outcome::Error((Status::Unauthorized, ()))
                    },
                }
            },
            _ => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

fn decode_token(token: &str, secret: &str) -> Result<TokenData<Claims>, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256)
    )
}
