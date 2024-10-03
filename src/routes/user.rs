use crate::controllers::user_controller::*;


pub fn user_routes() -> Vec<rocket::Route> {
    routes![ create_post, get_data, delete_user, edit_user, login]
}
