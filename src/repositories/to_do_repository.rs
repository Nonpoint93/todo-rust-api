//! # To-Do Repository Module
//!
//! Handles all database interaction and queries related to to-do items using Diesel ORM.

use crate::configuration::schema::to_do_table::{self, txt_status, txt_title};
use crate::models::entities::items::item::Item;
use crate::models::entities::items::new_item::NewItem;
use diesel::prelude::*;

pub trait ToDoRepositoryTrait {
    fn find_all(&mut self) -> Result<Vec<Item>, String>;
    fn insert(&mut self, new_item: &NewItem) -> Result<usize, String>;
    fn update_status(&mut self, title: &str, status: &str) -> Result<usize, String>;
    fn delete_by_title(&mut self, title: &str) -> Result<usize, String>;
}

/// Retrieves all to-do items from the database, ordered by ID ascending.
pub fn find_all(connection: &mut PgConnection) -> Result<Vec<Item>, diesel::result::Error> {
    to_do_table::table
        .order(to_do_table::columns::id.asc())
        .load::<Item>(connection)
}

/// Inserts a new to-do item into the database.
pub fn insert(
    connection: &mut PgConnection,
    new_item: &NewItem,
) -> Result<usize, diesel::result::Error> {
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
    let results: Vec<Item> =
        diesel::QueryDsl::filter(to_do_table::table, txt_title.eq(title)).load(connection)?;

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
pub fn delete_by_title(
    connection: &mut PgConnection,
    title: &str,
) -> Result<usize, diesel::result::Error> {
    let items: Vec<Item> =
        diesel::QueryDsl::filter(to_do_table::table, txt_title.eq(title)).load(connection)?;

    if items.is_empty() {
        return Ok(0);
    }

    diesel::delete(&items[0]).execute(connection)
}

#[cfg(test)]
pub struct MockToDoRepository {
    pub items: Vec<Item>,
}

#[cfg(test)]
impl ToDoRepositoryTrait for MockToDoRepository {
    fn find_all(&mut self) -> Result<Vec<Item>, String> {
        Ok(self.items.clone())
    }

    fn insert(&mut self, new_item: &NewItem) -> Result<usize, String> {
        let new_id = (self.items.len() as i32) + 1;
        let item = Item {
            id: new_id,
            txt_title: new_item.txt_title.clone(),
            txt_status: new_item.txt_status.clone(),
            dat_date: new_item.dat_date,
        };
        self.items.push(item);
        Ok(1)
    }

    fn update_status(&mut self, title: &str, status: &str) -> Result<usize, String> {
        let mut updated = 0;
        for item in &mut self.items {
            if item.txt_title == title {
                item.txt_status = status.to_string();
                updated += 1;
            }
        }
        Ok(updated)
    }

    fn delete_by_title(&mut self, title: &str) -> Result<usize, String> {
        let initial_len = self.items.len();
        self.items.retain(|item| item.txt_title != title);
        let deleted = initial_len - self.items.len();
        Ok(deleted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::entities::items::new_item::NewItem;
    use chrono::Utc;

    #[test]
    fn test_mock_repository_workflow() {
        let mut mock_repo = MockToDoRepository { items: vec![] };

        let new_task = NewItem {
            txt_title: String::from("Buy a Coffee"),
            txt_status: String::from("PENDING"),
            dat_date: Utc::now().naive_utc(),
        };

        let insert_res = mock_repo.insert(&new_task);
        assert!(insert_res.is_ok());

        let items = mock_repo.find_all().unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].txt_title, "Buy a Coffee");
        assert_eq!(items[0].txt_status, "PENDING");

        let update_res = mock_repo.update_status("Buy a Coffee", "DONE");
        assert!(update_res.is_ok());

        let items_updated = mock_repo.find_all().unwrap();
        assert_eq!(items_updated[0].txt_status, "DONE");

        let delete_res = mock_repo.delete_by_title("Buy a Coffee");
        assert!(delete_res.is_ok());

        let items_final = mock_repo.find_all().unwrap();
        assert!(items_final.is_empty());
    }
}
