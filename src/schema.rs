// @generated automatically by Diesel CLI.

diesel::table! {
    to_do_table (id) {
        id -> Int4,
        title -> Varchar,
        status -> Varchar,
        date -> Timestamp,
    }
}
