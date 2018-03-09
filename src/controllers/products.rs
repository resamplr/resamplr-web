//! Controller for managing products

use db::Conn as DbConn;
use rocket_contrib::{Json, Value};
use super::models::products::Product;

#[get("/products", format = "application/json")]
fn index(conn: DbConn) -> Json {
    Json(json!({
        Product::all(&conn)
    }))
}