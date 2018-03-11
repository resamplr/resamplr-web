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
    pub description: Option<String>,
    pub published: bool
}
