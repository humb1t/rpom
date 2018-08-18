use db_pool;
use diesel;
use diesel::prelude::*;
use rocket_contrib::Json;
use schema::orders;

#[derive(Serialize, Deserialize, Queryable, Insertable, Copy, Clone)]
#[table_name = "orders"]
pub struct Order {
    pub id: i32,
    pub specification_id: i32,
    pub quantity: i32,
}

#[post("/", data = "<order>")]
fn create(order: Json<Order>, conn: db_pool::DbConn) -> Json<Order> {
    use schema::orders::dsl::*;
    let order: Order = order.0;
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

#[get("/<fid>")]
fn get(fid: i32, conn: db_pool::DbConn) -> Json<Order> {
    use schema::orders::dsl::*;
    let order = orders
        .find(fid)
        .first::<Order>(&*conn)
        .expect("Error loading order");

    Json(order)
}

#[get("/")]
fn get_all(conn: db_pool::DbConn) -> QueryResult<Json<Vec<Order>>> {
    use schema::orders::dsl::*;
    orders.load::<Order>(&*conn)
        .map(|ordrs| Json(ordrs))
}
