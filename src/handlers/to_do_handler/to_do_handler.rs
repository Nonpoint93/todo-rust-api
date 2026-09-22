//! # To-Do Views Factory Module
//! 
//! Configures and registers all HTTP routing scopes and endpoints related 
//! to the To-Do feature (`/to_do/v1/item`).

use actix_web::web::{ServiceConfig, get, post, scope, delete, put};

use crate::services::to_do_service;


/// Registers all to-do related routes and handlers into the Actix-web service configuration.
/// 
/// - `GET /to_do/v1/item`: Retrieves the summary list of all tasks.
/// - `POST /to_do/v1/item/create/{title}`: Creates a new task with the given title.
/// - `PUT /to_do/v1/item/edit`: Edits an existing task status.
/// - `DELETE /to_do/v1/item/delete`: Deletes a task.
pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/to_do/v1/item")
            .route("", get().to(to_do_service::get))
            .route("create/{title}", post().to(to_do_service::create))
            .route("edit", put().to(to_do_service::edit))
            .route("delete", delete().to(to_do_service::delete))
    );
}