// @generated automatically by Diesel CLI.

diesel::table! {
    items (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        price -> Int4,
        instock -> Int4,
    }
}
