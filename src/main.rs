#![allow(non_snake_case)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate chrono;

use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;

pub mod repo;
pub mod entities;
pub mod models;
pub mod schema;
pub mod controllers;
pub mod utils;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = "postgres://postgres:1@localhost/AuthApi".to_string();

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[launch]
fn rocket() -> _ { 
    rocket::build().mount("/", routes![controllers::register, controllers::login, controllers::get_info])
}