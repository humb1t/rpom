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

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    rocket::ignite().manage(db_pool::init_pool(&database_url)).mount(
        "/",
        routes![
           order::create,
           order::get,
           order::get_all
         ],
    ).launch();
}