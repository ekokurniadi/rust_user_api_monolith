use rocket::Route;
use crate::handler::*;


pub fn user_routes()->Vec<Route>{
    routes![
        user_handler::get_users,
        user_handler::create_user,
        user_handler::update_user,
        user_handler::delete_user,
        user_handler::login_user,
    ]
}