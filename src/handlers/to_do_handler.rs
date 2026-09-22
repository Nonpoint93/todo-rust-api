//! # To-Do Views Factory Module
//!
//! Configures and registers all HTTP routing scopes and endpoints related
//! to the To-Do feature (`/to_do/v1/item`).

use actix_web::web::{delete, get, post, put, scope, ServiceConfig};

use crate::services::to_do_service;

/// Registers all to-do related routes and handlers into the Actix-web service configuration.
///
/// - `GET /to_do/v1/item`: Retrieves the summary list of all tasks.
/// - `POST /to_do/v1/item/create`: Creates a new task using a JSON body payload.
/// - `PUT /to_do/v1/item/edit/{title}`: Edits an existing task's status identified by its title.
/// - `DELETE /to_do/v1/item/delete/{title}`: Deletes a task identified by its title.
pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/to_do/v1/item")
            .route("", get().to(to_do_service::get))
            .route("create", post().to(to_do_service::create))
            .route("edit/{title}", put().to(to_do_service::edit))
            .route("delete/{title}", delete().to(to_do_service::delete)),
    );
}
