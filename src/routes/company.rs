use crate::controllers::company_controller::*;


pub fn company_routes() -> Vec<rocket::Route> {
    routes![create_data ]
}
