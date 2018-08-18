#![feature(plugin)]
#![feature(decl_macro)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use dotenv::dotenv;
use std::env;

pub mod order;
pub mod specification;
pub mod db_pool;
pub mod schema;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    rocket::ignite()
        .manage(
            db_pool::init_pool(&database_url)
        )
        .mount(
            "/orders",
        routes![
           order::create,
           order::get,
           order::get_all
         ],
        )
        .mount(
            "/specifications",
            routes![
                specification::create,
                specification::get,
                specification::get_all
            ],
        )
        .launch();
}