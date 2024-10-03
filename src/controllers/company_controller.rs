use rocket::{http, serde::json::Json};
use rocket::http::Status;
use crate::middleware::jwt_authorization::JwtToken;
use std::env;
use crate::services::company_service;
use jsonwebtoken::{encode, EncodingKey, Header};
use crate::models::company::{Company, NewCompany};

#[post("/company", format = "json", data = "<company>")]
pub fn create_data(token:JwtToken, company: Json<NewCompany>) -> Result<Json<Company>, Status> {
    let data_company = company.into_inner();
    if token.0.id == data_company.owner_id {
        let company = company_service::add(data_company);
        Ok(Json(company)) 
    }
    else {
        Err(Status::Forbidden) 
    }
  
}

#[get("/company/list")]
pub fn get_data ()-> Result<Json<Vec<Company>>, Status> {
    let data = company_service::get_all();
    Ok(Json(data))
}

#[delete("/company/<id>")]
pub fn delete_user(id: i32) -> Result<String, rocket::http::Status> {
    company_service::delete(id).map(|_| "sukses hapus".to_string())
        .map_err(|_| rocket::http::Status::NotFound)
}

#[patch("/company/<id>", format="json", data="<company>")]
pub fn update(company: Json<NewCompany>, id: i32)-> Result<Json<Vec<Company>>, Status> {
    let data = company_service::update(company, id);
        Ok(Json(data))
}

