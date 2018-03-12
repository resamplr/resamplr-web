use schema::products;

// Base data structure for a product
#[derive(Deserialize, Serialize, Queryable)]
#[primary_key(id)]
#[table_name = "products"]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub published: bool,
}

/// Used when creating products
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "products"]
pub struct NewProduct {
    pub name: String,
    pub description: Option<String>,
    pub price_cents: i32,
    pub published: bool,
}

/// Used when updating products
#[derive(AsChangeset, Identifiable, Deserialize, Serialize)]
#[table_name = "products"]
pub struct ProductForm {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price_cents: Option<i32>,
    pub published: Option<bool>,
}
