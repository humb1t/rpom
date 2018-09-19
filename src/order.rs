use db_pool;
use diesel;
use diesel::deserialize::Queryable;
use diesel::insertable::Insertable;
use diesel::prelude::*;
use product::*;
use rocket_contrib::Json;
use schema::{orders, products};
use specification::Specification;

#[derive(Serialize, Deserialize, Queryable, Associations, Copy, Clone, Debug)]
#[belongs_to(Specification)]
pub struct Order {
    pub id: i32,
    pub specification_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub status: OrderStatus,
}

#[derive(Deserialize, Insertable)]
#[table_name = "orders"]
pub struct NewOrder {
    pub specification_id: i32,
    pub product_id: i32,
    pub quantity: i32,
    pub status: OrderStatus,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, DbEnum)]
#[PgType = "order_status"]
#[DieselType = "Order_status"]
pub enum OrderStatus {
    Entering,
    InProgress,
    Cancelled,
    Completed,
}

#[post("/", data = "<order>")]
fn create(order: Json<NewOrder>, conn: db_pool::DbConn) -> Json<Order> {
    use schema::orders::dsl::*;
    use schema::products::dsl::*;
    let mut order: NewOrder = order.0;
    let product: Product = diesel::insert_into(products)
        .values(
            NewProduct {
                specification_id: order.specification_id
            }
        )
        .get_result(&*conn)
        .expect("Error saving new product");
    order.product_id = product.id;
    Json(
        diesel::insert_into(orders)
            .values(&order)
            .get_result(&*conn)
            .expect("Error saving new order")
    )
}

#[get("/<fid>")]
fn get(fid: i32, conn: db_pool::DbConn) -> Json<Order> {
    use schema::orders::dsl::*;
    Json(
        orders
            .find(fid)
            .first::<Order>(&*conn)
            .expect("Error loading order")
    )
}

#[post("/<fid>/start")]
fn start(fid: i32, conn: db_pool::DbConn) -> Json<Order> {
    use schema::orders::dsl::*;
    let order = diesel::update(
        orders
            .find(fid)
    ).set(status.eq(OrderStatus::InProgress))
        .get_result::<Order>(&*conn)
        .expect("Error loading order");
    println!("{:?} has been started", order);
    Json(order)
}

#[post("/<fid>/cancel")]
fn cancel(fid: i32, conn: db_pool::DbConn) -> Json<Order> {
    use schema::orders::dsl::*;
    let order = diesel::update(
        orders
            .find(fid)
    ).set(status.eq(OrderStatus::Cancelled))
        .get_result::<Order>(&*conn)
        .expect("Error loading order");
    println!("{:?} has been cancelled", order);
    Json(order)
}

#[get("/")]
fn get_all(conn: db_pool::DbConn) -> QueryResult<Json<Vec<Order>>> {
    use schema::orders::dsl::*;
    orders.load::<Order>(&*conn)
        .map(|all_orders| Json(all_orders))
}
