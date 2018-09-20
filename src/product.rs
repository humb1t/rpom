use db_pool;
use diesel;
use diesel::deserialize::Queryable;
use diesel::insertable::Insertable;
use diesel::prelude::*;
use order::Order;
use rocket_contrib::Json;
use schema::{orders, products, specifications};
use specification::Specification;

#[derive(Serialize, Deserialize,Identifiable, Queryable, Associations, Clone)]
#[belongs_to(Specification)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub specification_id: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub specification_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ProductWithOrdersAndSpecification {
    pub id: i32,
    pub specification: Specification,
    pub bellonging_orders: Vec<Order>,
}

#[get("/<fid>")]
fn get(fid: i32, conn: db_pool::DbConn) -> Json<ProductWithOrdersAndSpecification> {
    use schema::products::dsl::*;
    use schema::specifications::dsl::*;
    use schema::orders::dsl::*;
    let product = products
        .find(fid)
        .first::<Product>(&*conn)
        .expect("Error loading product");
    let specification = specifications
        .find(product.specification_id)
        .first::<Specification>(&*conn)
        .expect("Error loading product's specification");
    let orders_list = Order::belonging_to(&product)
        .load::<Order>(&*conn)
        .expect("Error loading product's orders");

    Json(
        ProductWithOrdersAndSpecification {
            id: product.id,
            specification: specification,
            bellonging_orders: orders_list,
        }
    )
}

#[get("/")]
fn get_all(conn: db_pool::DbConn) -> QueryResult<Json<Vec<Product>>> {
    use schema::products::dsl::*;
    products.load::<Product>(&*conn)
        .map(|prdcts| Json(prdcts))
}
