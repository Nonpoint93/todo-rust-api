
use actix_web::web::{get, scope, ServiceConfig};

use crate::services::to_do_service;

pub fn to_do_views_factory(app: &mut ServiceConfig){
    app.service(scope("/to_do/v1/item")
    .route("create/{title}", get().to(to_do_service::get)));
}