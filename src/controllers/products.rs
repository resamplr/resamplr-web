//! Controller for managing products
use db::Conn as DbConn;
use rocket_contrib::Json;
use models::product::{Product, NewProduct, ProductForm};
use schema::products;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel;
use rocket::response::status::NoContent;
use diesel::prelude::*;
use schema::products::dsl::*;

/// Get a product via `id`
#[get("/api/v1/products/<product_id>", format = "application/json")]
fn get(product_id: i32, conn: DbConn) -> Result<Json<Product>, diesel::result::Error> {
    // query the database for a product matching our `id`
    let product_request = products.find(product_id)
                                  .get_result(&*conn)?;
    Ok(Json(product_request))
}

/// Get all products
#[get("/api/v1/products", format = "application/json")]
fn index(conn: DbConn) -> Result<Json<Vec<Product>>, diesel::result::Error> {
    // get all products in database
    let products_request = products.load(&*conn)?;
    Ok(Json(products_request))
}

/// Create a new product
#[post("/api/v1/products", format = "application/json", data = "<product>")]
fn create(product: Json<NewProduct>, conn: DbConn) -> Result<Json<Product>, diesel::result::Error> {
    let new_product = diesel::insert_into(products)
                        .values(&product.into_inner())
                        .get_result(&*conn)?;
    Ok(Json(new_product))
}

/// Destroy a product via `id`
#[delete("/api/v1/products/<product_id>", format = "application/json")]
fn delete(product_id: i32, conn: DbConn) -> Result<NoContent, diesel::result::Error> {
    // query the database for a product matching our `id`
    diesel::delete(products.filter(id.eq(&product_id)))
        .execute(&*conn)?;
    // TODO: Error if already deleted
    Ok(NoContent)
}

/// Update a product
#[patch("/api/v1/products", format = "application/json", data = "<product>")]
fn update(product: Json<ProductForm>, conn: DbConn) -> Result<Json<Product>, diesel::result::Error> {
    let edited_product = product.into_inner().save_changes(&*conn)?;
    Ok(Json(edited_product))
}