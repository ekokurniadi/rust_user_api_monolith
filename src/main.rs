#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use routes::routes;
use services::users::{IUserRepository, UserService};

use crate::db::init_pool;
mod db;
mod models;
mod schema;
mod services;
mod handler;
mod shared;
mod routes;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv().ok();

    let db_pool = init_pool();
    rocket::build()
        .manage(db_pool.clone())
        .manage(UserService::new(db_pool.clone()))
        .mount("/api/v1", routes())
        .launch()
        .await?;

    Ok(())
}
