use db_pool;
use diesel;
use diesel::deserialize::Queryable;
use diesel::insertable::Insertable;
use diesel::prelude::*;
use rocket_contrib::Json;
use schema::products;
use specification::Specification;

#[derive(Serialize, Deserialize, Queryable, Associations, Clone)]
#[belongs_to(Specification)]
pub struct Product {
    pub id: i32,
    pub specification_id: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub specification_id: i32,
}

#[get("/<fid>")]
fn get(fid: i32, conn: db_pool::DbConn) -> Json<Product> {
    use schema::products::dsl::*;
    let product = products
        .find(fid)
        .first::<Product>(&*conn)
        .expect("Error loading product");

    Json(product)
}

#[get("/")]
fn get_all(conn: db_pool::DbConn) -> QueryResult<Json<Vec<Product>>> {
    use schema::products::dsl::*;
    products.load::<Product>(&*conn)
        .map(|prdcts| Json(prdcts))
}
