table! {
    products (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Text>,
        price_cents -> Int4,
        published -> Bool,
    }
}
