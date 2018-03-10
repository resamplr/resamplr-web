use diesel::prelude::*;
use diesel;
use diesel::pg::PgConnection;
use schema::products;

// Base data structure for a product
#[derive(Queryable)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
}

/// Used when creating products
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
	pub name: String,
	pub description: String
}

impl Product {
	pub fn show(id: i32, conn: &PgConnection) -> Vec<Product> {
		products.find(id).load::<Product>(conn)
						 .expect("Couldn't find a product with that ID.")
	}

	pub fn all(conn: &PgConnection) -> Vec<Product> {
		// TODO: custom predicate for order
		products.order(products::id.desc())
				.load::<Product>(conn)
				.expect("Error loading products.")
	}
}