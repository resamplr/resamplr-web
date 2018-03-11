//! Controller for managing products
use db::Conn as DbConn;
use rocket_contrib::Json;
use models::product::{Product, NewProduct};
use schema::products;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;

/// Get a product via `id`
#[get("/api/v1/products/<id>", format = "application/json")]
fn get(id: i32, conn: DbConn) -> Result<Json<Product>, diesel::result::Error> {
	// query the database for a product matching our `id`
	let product_request = products::table
							.find(id)
							.first::<Product>(&*conn)?;
	Ok(Json(product_request))
}

/// Get all products
#[get("/api/v1/products", format = "application/json")]
fn index(conn: DbConn) -> Result<Json<Vec<Product>>, diesel::result::Error> {
	// get all products in database
	let products_request = products::table.load(&*conn)?;
	Ok(Json(products_request))
}

/// Create a new product
#[post("/api/v1/products", format = "application/json", data = "<product>")]
fn create(product: Json<NewProduct>, conn: DbConn) -> Result<Json<Product>, diesel::result::Error> {
	let new_product = diesel::insert_into(products::table)
						.values(&product.into_inner())
						.get_result(&*conn)?;

	Ok(Json(new_product))
}