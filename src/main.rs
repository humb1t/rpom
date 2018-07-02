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
use diesel::result::QueryResult;
use domain::Order;
use dotenv::dotenv;
use rocket_contrib::Json;
use std::env;

pub mod schema;

mod db_pool {
    use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
    use diesel::sqlite::SqliteConnection;
    use std::ops::Deref;

    pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

    pub struct DbConn(pub PooledConnection<ConnectionManager<SqliteConnection>>);

    pub fn init_pool(database_url: &str) -> SqlitePool {
        let manager =
            ConnectionManager::<SqliteConnection>::new(database_url);
        Pool::new(manager).expect("db pool")
    }

    impl Deref for DbConn {
        type Target = SqliteConnection;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}

mod web {
    use db_pool;
    use rocket::{Outcome, Request, State};
    use rocket::http::Status;
    use rocket::request::{self, FromRequest};

    /// Attempts to retrieve a single connection from the managed database pool. If
    /// no pool is currently managed, fails with an `InternalServerError` status. If
    /// no connections are available, fails with a `ServiceUnavailable` status.
    impl<'a, 'r> FromRequest<'a, 'r> for db_pool::DbConn {
        type Error = ();

        fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
            let pool = request.guard::<State<db_pool::SqlitePool>>()?;
            match pool.get() {
                Ok(conn) => Outcome::Success(db_pool::DbConn(conn)),
                Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
            }
        }
    }
}

mod domain {
    use diesel::prelude::*;
    use schema::orders;

    #[derive(Serialize, Deserialize, Queryable, Insertable, Copy, Clone)]
    #[table_name = "orders"]
    pub struct Order {
        pub id: i32,
        pub specification_id: i32,
        pub quantity: i32,
    }
}

#[post("/orders", data = "<order>")]
fn order_create(order: Json<Order>, conn: db_pool::DbConn) -> Json<Order> {
    use schema::orders::dsl::*;
    let order = order.0;
    let pid: i32 = order.id;
    diesel::insert_into(orders)
        .values(&order)
        .execute(&*conn)
        .expect("Error saving new order");
    let order = orders
        .find(pid)
        .first::<Order>(&*conn)
        .expect("Error loading order");

    Json(order)
}

#[get("/orders/<fid>")]
fn order_get(fid: i32, conn: db_pool::DbConn) -> Json<Order> {
    use schema::orders::dsl::*;
    let order = orders
        .find(fid)
        .first::<Order>(&*conn)
        .expect("Error loading order");

    Json(order)
}

#[get("/orders")]
fn order_get_all(conn: db_pool::DbConn) -> QueryResult<Json<Vec<Order>>> {
    use schema::orders::dsl::*;
    orders.load::<Order>(&*conn)
        .map(|ordrs| Json(ordrs))
}

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    rocket::ignite().manage(db_pool::init_pool(&database_url)).mount(
        "/",
        routes![
           order_create,
           order_get,
           order_get_all
         ],
    ).launch();
}