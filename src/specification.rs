use db_pool;
use diesel;
use diesel::prelude::*;
use rocket_contrib::Json;
use schema::specifications;

#[derive(Serialize, Deserialize, Queryable, AsChangeset, Insertable, Clone)]
#[table_name = "specifications"]
pub struct Specification {
    pub id: i32,
    pub name: String,
}

#[post("/", data = "<specification>")]
fn create(specification: Json<Specification>, conn: db_pool::DbConn) -> Json<Specification> {
    use schema::specifications::dsl::*;
    let specification: Specification = specification.0;
    let pid: i32 = specification.id;
    diesel::insert_into(specifications)
        .values(&specification)
        .execute(&*conn)
        .expect("Error saving new specification");
    let specification = specifications
        .find(pid)
        .first::<Specification>(&*conn)
        .expect("Error loading specification");

    Json(specification)
}

#[get("/<fid>")]
fn get(fid: i32, conn: db_pool::DbConn) -> Json<Specification> {
    use schema::specifications::dsl::*;
    let specification = specifications
        .find(fid)
        .first::<Specification>(&*conn)
        .expect("Error loading specification");

    Json(specification)
}

#[get("/")]
fn get_all(conn: db_pool::DbConn) -> QueryResult<Json<Vec<Specification>>> {
    use schema::specifications::dsl::*;
    specifications.load::<Specification>(&*conn)
        .map(|list| Json(list))
}

#[patch("/<fid>", data = "<specification>")]
fn update(fid: i32, specification: Json<Specification>, conn: db_pool::DbConn) -> Json<Specification> {
    use schema::specifications::dsl::*;
    diesel::update(
        specifications.find(fid)
    ).set(specification.into_inner())
        .execute(&*conn)
        .expect("Error updating specification");
    let result = specifications.find(fid)
        .first::<Specification>(&*conn)
        .expect("Error loading specification");

    Json(result)
}

#[delete("/<fid>")]
fn delete(fid: i32, conn: db_pool::DbConn) {
    use schema::specifications::dsl::*;
    diesel::delete(
        specifications.find(fid)
    ).execute(&*conn)
        .expect("Error deleting specification");
}
