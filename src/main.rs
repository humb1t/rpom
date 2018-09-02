#![feature(plugin)]
#![feature(decl_macro)]
#![plugin(rocket_codegen)]

extern crate crossbeam;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use crossbeam::queue::MsQueue;
use dotenv::dotenv;
use std::env;

pub mod order;
pub mod product;
pub mod specification;
pub mod db_pool;
pub mod opf;
pub mod schema;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    rocket::ignite()
        .manage(db_pool::init_pool(&database_url))
        .manage(opf::OrdersChannel::new(MsQueue::new()))
        .mount(
            "/orders",
            routes![
           order::create,
           order::get,
           order::start,
           order::cancel,
           order::get_all
         ],
        )
        .mount(
            "/products",
            routes![
           product::create,
           product::get,
           product::get_all
         ],
        )
        .mount(
            "/specifications",
            routes![
                specification::create,
                specification::get,
                specification::find,
                specification::get_all,
                specification::update,
                specification::delete
            ],
        )
        .launch();
}