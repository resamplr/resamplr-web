//! Controller for managing products
use db::Conn as DbConn;
use rocket_contrib::{Json, Value};
use models::product::Product;
use schema::products;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;

#[get("/api/v1/products/<id>", format = "application/json")]
fn get(id: i32, conn: DbConn) -> Result<Json<Product>, diesel::result::Error> {
	// query the database for a product matching our `id`
	let product_request = products::table
							.find(id)
							.first::<Product>(&*conn)?;
	Ok(Json(product_request))
}

#[get("/api/v1/products", format = "application/json")]
fn index(conn: DbConn) -> Result<Json<Vec<Product>>, diesel::result::Error> {
	// get all products in database
	let products_request = products::table.load(&*conn)?;
	Ok(Json(products_request))
}