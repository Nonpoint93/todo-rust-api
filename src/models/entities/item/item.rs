use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::configuration::schema::to_do_table)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Item {
    pub id: i32,
    pub txt_title: String,
    pub txt_status: String,
    pub dat_date: NaiveDateTime
}