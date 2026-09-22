//! # To-Do Repository Module
//! 
//! Handles all database interaction and queries related to to-do items using Diesel ORM.

use crate::configuration::schema::to_do_table::{self, txt_status, txt_title};
use crate::models::entities::item::item::Item;
use crate::models::entities::item::new_item::NewItem;
use diesel::prelude::*;

/// Retrieves all to-do items from the database, ordered by ID ascending.
pub fn find_all(connection: &mut PgConnection) -> Result<Vec<Item>, diesel::result::Error> {
    to_do_table::table
        .order(to_do_table::columns::id.asc())
        .load::<Item>(connection)
}

/// Inserts a new to-do item into the database.
pub fn insert(connection: &mut PgConnection, new_item: &NewItem) -> Result<usize, diesel::result::Error> {
    diesel::insert_into(to_do_table::table)
        .values(new_item)
        .execute(connection)
}

/// Updates the status of a to-do item by its title.
pub fn update_status_by_title(
    connection: &mut PgConnection,
    title: &str,
    status: &str,
) -> Result<usize, diesel::result::Error> {
    let results: Vec<Item> = diesel::QueryDsl::filter(
        to_do_table::table, txt_title.eq(title)
    ).load(connection)?;

    let mut affected_rows = 0;
    for result in results {
        let count = diesel::update(to_do_table::table.find(result.id))
            .set(txt_status.eq(status))
            .execute(connection)?;
        affected_rows += count;
    }

    Ok(affected_rows)
}

/// Deletes a to-do item by its title.
pub fn delete_by_title(connection: &mut PgConnection, title: &str) -> Result<usize, diesel::result::Error> {
    let items: Vec<Item> = diesel::QueryDsl::filter(
        to_do_table::table, txt_title.eq(title)
    ).load(connection)?;

    if items.is_empty() {
        return Ok(0);
    }

    diesel::delete(&items[0]).execute(connection)
}