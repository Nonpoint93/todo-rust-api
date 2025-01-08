use actix_web::web::ServiceConfig;
use to_do_handler::to_do_handler::to_do_views_factory;

pub mod to_do_handler;


pub fn views_factory(app: &mut ServiceConfig){
    to_do_views_factory(app);
}