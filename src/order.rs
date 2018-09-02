use db_pool;
use diesel;
use diesel::prelude::*;
use opf::OrderAction;
use opf::OrderEvent;
use opf::OrdersChannel;
use rocket::State;
use rocket_contrib::Json;
use schema::orders;

#[derive(Serialize, Deserialize, Queryable, Copy, Clone, Debug)]
pub struct Order {
    pub id: i32,
    pub specification_id: i32,
    pub quantity: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "orders"]
pub struct NewOrder {
    pub specification_id: i32,
    pub quantity: i32,
}

pub enum Status {
    Entering,
    InProgress,
    Cancelled,
    Completed,
}

#[post("/", data = "<order>")]
fn create(order: Json<NewOrder>, conn: db_pool::DbConn) {
    use schema::orders::dsl::*;
    let order: NewOrder = order.0;
    diesel::insert_into(orders)
        .values(&order)
        .execute(&*conn)
        .expect("Error saving new order");
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

#[post("/<fid>/start")]
fn start(fid: i32, conn: db_pool::DbConn) -> Json<Order> {
    use schema::orders::dsl::*;
    let order = orders
        .find(fid)
        .first::<Order>(&*conn)
        .expect("Error loading order");
    println!("{:?} has been started", order);
    Json(order)
}

#[post("/<fid>/cancel")]
fn cancel(fid: i32, conn: db_pool::DbConn, orders_channel: State<OrdersChannel>) -> Json<Order> {
    use schema::orders::dsl::*;
    let order: Order = orders
        .find(fid)
        .first::<Order>(&*conn)
        .expect("Error loading order");
    orders_channel.push(
        OrderEvent {
            action: OrderAction::New,
            quantity: order.quantity,
            specification_id: order.specification_id,
        }
    );
    println!("{:?} has been cancelled", order);
    Json(order)
}

#[get("/")]
fn get_all(conn: db_pool::DbConn) -> QueryResult<Json<Vec<Order>>> {
    use schema::orders::dsl::*;
    orders.load::<Order>(&*conn)
        .map(|ordrs| Json(ordrs))
}
