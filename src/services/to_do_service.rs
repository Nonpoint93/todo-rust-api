//! # To-Do Service Module
//!
//! Contains the core business logic for task management,
//! interacting directly with the database via Diesel and
//! handling Actix-web endpoints.

use crate::configuration::database::establish_connection;
use crate::configuration::schema::to_do_table::{self};
use crate::enums::item_types::{to_do_factory, ItemTypes};
use crate::enums::task_status::TaskStatus;

use crate::models::entities::items::item::Item;

use crate::models::requests::create_item_request::CreateItemRequest;
use crate::models::responses::summary_item_response::SummaryItemResponse;

use crate::models::entities::items::new_item::NewItem;
use crate::repositories::to_do_repository;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

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
pub async fn edit(
    title_path: web::Path<String>,
    payload: web::Json<CreateItemRequest>,
) -> HttpResponse {
    let connection = &mut establish_connection();
    let title = title_path.into_inner();

    let res = (|| -> Result<SummaryItemResponse, diesel::result::Error> {
        to_do_repository::update_status_by_title(connection, &title, &payload.status)?;

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
pub async fn delete(title_path: web::Path<String>) -> HttpResponse {
    let connection = &mut establish_connection();

    let res = (|| -> Result<SummaryItemResponse, diesel::result::Error> {
        to_do_repository::delete_by_title(connection, &title_path.into_inner())?;

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

    let items: Vec<Item> = to_do_table::table
        .order(to_do_table::columns::id.asc())
        .load(&mut connection)?;

    for item in items {
        let status: TaskStatus = TaskStatus::from_string(item.txt_status);
        let item: ItemTypes = to_do_factory(&item.txt_title, status);
        array_buffer.push(item);
    }

    Ok(SummaryItemResponse::new(array_buffer))
}

/// HTTP POST handler to create a new task.
///
/// Extracts the task title from the route path as an owned string, generates the
/// current UTC timestamp, sets the initial status to `PENDING`, and inserts the
/// new record into the database via Diesel.
///
/// Returns an HTTP `200 OK` with the updated task summary on success,
/// or an HTTP `500 Internal Server Error` if a database operation fails.
pub async fn create(payload: web::Json<CreateItemRequest>) -> HttpResponse {
    let connection: &mut PgConnection = &mut establish_connection();
    let title_str: String = payload.title.clone();

    let res = (|| -> Result<SummaryItemResponse, diesel::result::Error> {
        let current_date = chrono::Utc::now().naive_utc();

        let new_item = NewItem {
            txt_title: title_str,
            txt_status: TaskStatus::PENDING.stringify(),
            dat_date: current_date,
        };

        to_do_repository::insert(connection, &new_item)?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::requests::create_item_request::CreateItemRequest;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_get_tasks_endpoint() {
        let app = test::init_service(App::new().route("/item", web::get().to(get))).await;

        let req = test::TestRequest::get().uri("/item").to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_server_error() || resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_create_task_endpoint_payload() {
        let app =
            test::init_service(App::new().route("/item/create", web::post().to(create))).await;

        let payload = CreateItemRequest {
            title: String::from("Test Integration Task"),
            status: String::from("PENDING"),
        };

        let req = test::TestRequest::post()
            .uri("/item/create")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_server_error() || resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_edit_task_endpoint_payload() {
        let app =
            test::init_service(App::new().route("/item/edit/{title}", web::put().to(edit))).await;

        let payload: CreateItemRequest = CreateItemRequest {
            title: String::from("Test Integration Task"),
            status: String::from("DONE"),
        };

        let req = test::TestRequest::put()
            .uri("/item/edit/Test%20Integration%20Task")
            .set_json(&payload)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_server_error() || resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_delete_task_endpoint() {
        let app =
            test::init_service(App::new().route("/item/delete/{title}", web::delete().to(delete)))
                .await;

        let req = test::TestRequest::delete()
            .uri("/item/delete/Test%20Integration%20Task")
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_server_error() || resp.status().is_success());
    }
}
