//! # To-Do Service Module
//! 
//! Contains the core business logic for task management,
//! interacting directly with the database via Diesel and 
//! handling Actix-web endpoints.

use crate::configuration::database::establish_connection;
use crate::configuration::schema::to_do_table::{self, txt_status, txt_title};
use crate::enums::item_types::{to_do_factory, ItemTypes};
use crate::enums::task_status::TaskStatus;
use crate::models::done::Done;
use crate::models::entities::item::item::Item;
use crate::models::entities::item::new_item::NewItem;
use crate::models::jwtoken::JwToken;
use crate::models::pending::Pending;
use crate::models::responses::get_item_response::GetItemResponse;
use crate::models::responses::summary_item_response::SummaryItemResponse;
use crate::models::traits::create::Create;
use crate::models::traits::delete::Delete;
use crate::models::traits::edit::Edit;
use crate::models::traits::get::Get;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder};
use serde_json::Value;
use serde_json::Map;

/// HTTP GET handler to retrieve the summary of all tasks.
/// 
/// Returns a JSON containing the current state of items or a 500 Internal Server Error
/// if the database connection fails.
pub async fn get() -> impl Responder {
    match get_state() {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => {
            eprintln!("[!] Database error {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// HTTP handler to edit the status of an existing task.
/// 
/// Requires a valid JWT token and a JSON body with the item title 
/// and its new status.
pub async fn edit(get_item_response: web::Json<GetItemResponse>, token: JwToken) -> HttpResponse {
    println!("[+] Here is the message in the token: {}", token.message);

    let connection = &mut establish_connection();

    let res = (|| -> Result<SummaryItemResponse, diesel::result::Error> {
        let results: Vec<Item> = diesel::QueryDsl::filter(
            to_do_table::table, txt_title.eq(&get_item_response.title)
        ).load(connection)?;

        for result in results {
            diesel::update(to_do_table::table.find(result.id))
            .set(txt_status.eq(&get_item_response.status))
            .returning(Item::as_returning())
            .get_result(connection)?;
        }

        get_state()
    })();

    match res {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => {
            eprintln!("[!] Database error {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// HTTP handler to delete a task by its title.
/// 
/// Requires an authorized JWT token.
pub async fn delete(get_item_response: web::Json<GetItemResponse>, _token: JwToken) -> HttpResponse {
    let connection = &mut establish_connection();

    let res = (|| -> Result<SummaryItemResponse, diesel::result::Error> {
        let items: Vec<Item> = diesel::QueryDsl::filter(
            to_do_table::table, txt_title.eq(&get_item_response.title)
        ).load(connection)?;

        let _ = diesel::delete(&items[0]).execute(connection);

        get_state()
    })();

    match res {
        Ok(summary) => HttpResponse::Ok().json(summary),
        Err(err) => {
            eprintln!("[!] Database error {:?}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

/// Queries the database and constructs a `SummaryItemResponse` 
/// containing all tasks ordered by ID in ascending order.
pub fn get_state() -> Result<SummaryItemResponse, diesel::result::Error> {
    let mut array_buffer: Vec<crate::enums::item_types::ItemTypes> = Vec::new();
    let mut connection: diesel::PgConnection = establish_connection();
    
    let items: Vec<Item> = to_do_table::table.order(to_do_table::columns::id.asc()).load(&mut connection)?;

    for item in items {
        let status: TaskStatus = TaskStatus::from_string(item.txt_status);
        let item: ItemTypes = to_do_factory(&item.txt_title, status);
        array_buffer.push(item);
    }
    
    Ok(SummaryItemResponse::new(array_buffer))
}

/// Processes specific commands for pending tasks.
fn process_pending(item: Pending, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "create" => item.create(&item.super_struct.title,
                                &item.super_struct.status.stringify(), &mut state),
        "edit" => item.set_to_done(&item.super_struct.title,
                                   &mut state),
        _ => println!("command: {} not supported", command)
    }
}

/// Processes specific commands for completed tasks.
fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command)
    }
}

/// Dispatches the received command to the corresponding task type (Pending or Done).
pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state)
    }
}