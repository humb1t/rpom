use db_pool;
use diesel;
use diesel::deserialize::Queryable;
use diesel::insertable::Insertable;
use diesel::prelude::*;
use rocket_contrib::Json;
use schema::products;
use specification::Specification;

#[derive(Serialize, Deserialize, Queryable, Insertable, Associations, Clone)]
#[table_name = "products"]
#[belongs_to(Specification)]
pub struct Product {
    pub id: i32,
    pub specification_id: i32,
}

#[post("/", data = "<product>")]
fn create(product: Json<Product>, conn: db_pool::DbConn) -> Json<Product> {
    use schema::products::dsl::*;
    let product: Product = product.0;
    let pid: i32 = product.id;
    diesel::insert_into(products)
        .values(&product)
        .execute(&*conn)
        .expect("Error saving new product");
    let product = products
        .find(pid)
        .first::<Product>(&*conn)
        .expect("Error loading product");

    Json(product)
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
