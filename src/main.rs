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

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::result::QueryResult;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket_contrib::Json;
use std::env;
use std::ops::Deref;

pub mod schema;

type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

fn init_pool() -> SqlitePool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager =
        ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::new(manager).expect("db pool")
}

pub struct DbConn(pub PooledConnection<ConnectionManager<SqliteConnection>>);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<SqlitePool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for DbConn {
    type Target = SqliteConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Queryable, Copy, Clone)]
struct Order {
    id: i32,
    specification_id: Option<i32>,
    quantity: Option<i32>,
}

#[post("/orders", data = "<order>")]
fn new_order(order: Json<Order>) -> Json<Order> {
    order
}

#[get("/orders/<id>")]
fn order_get(id: i32, conn: DbConn) -> Json<Order> {
    use schema::orders::dsl::*;
    let order = orders
        .find(id)
        .first::<Order>(&*conn)
        .expect("Error loading order");

    Json(order)
}

/*#[get("/orders")]
fn get_orders(conn: DbConn) -> Vec<Json<Order>> {
    use schema::orders::dsl::*;
    orders.load(&*conn)
        .unwrap()
        .iter()
        .map(|orders| Json(order))
        .collect::<Vec<_>>()
}*/

fn main() {
    rocket::ignite().manage(init_pool()).mount(
        "/",
        routes![
           new_order,
           order_get
         ],
    ).launch();
}