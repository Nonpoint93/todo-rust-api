use chrono::{NaiveDateTime, Utc};
use diesel::prelude::Insertable;

#[derive(Insertable)]
#[diesel(table_name = crate::configuration::schema::to_do_table)]
pub struct NewItem {
    pub txt_title: String,
    pub txt_status: String,
    pub dat_date: NaiveDateTime
}


impl NewItem {
    pub fn new(txt_title: String) -> NewItem {
        let now = Utc::now().naive_local();
        return NewItem{
            txt_title,
            txt_status: String::from("PENDING"),
            dat_date: now
        }
    }
}