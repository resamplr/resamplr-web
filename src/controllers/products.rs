//! Controller for managing products
use db::Conn as DbConn;
use rocket_contrib::{Json, Value};
use models::product::Product;

/// Get all products
#[get("/products", format = "application/json")]
fn index(conn: DbConn) -> Json {
    // Json(json!(Product::all(&conn)))
    // TODO 
}