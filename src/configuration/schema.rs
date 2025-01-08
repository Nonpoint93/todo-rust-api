// @generated automatically by Diesel CLI.

diesel::table! {
    to_do_table (id) {
        id -> Int4,
        txt_title -> Varchar,
        txt_status -> Varchar,
        dat_date -> Timestamp,
    }
}
