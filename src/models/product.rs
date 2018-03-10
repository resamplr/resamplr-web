use diesel::prelude::*;
use schema::products;

// Base data structure for a product
#[derive(Deserialize, Serialize, Identifiable, Queryable)]
#[primary_key(id)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub published: bool
}

/// Used when creating products
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
	pub name: String,
	pub description: Option<String>
}

// impl<'a> Product<'a> {
// 	pub fn show(id: i32, conn: &PgConnection) -> QueryResult<Self> {
// 		products::table
// 			.find(id)
// 			.first::<Product>(&*conn)
// 	}
// 
// 	pub fn all(conn: &PgConnection) -> QueryResult<Vec<Self>> {
// 		// TODO: custom predicate for order
// 		products::table
// 			//.order(products::id.desc())
// 			.load(&conn)
// 	}
// }