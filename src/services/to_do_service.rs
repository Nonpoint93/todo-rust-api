use crate::configuration::database::establish_connection;
use crate::configuration::schema::to_do_table::{self, id, txt_status, txt_title};
use crate::enums::item_types::{to_do_factory, ItemTypes};
use crate::enums::task_status::TaskStatus;
use crate::models::done::Done;
use crate::models::entities::item::item::Item;
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

pub async fn get() -> impl Responder{
    get_state()
}

pub async fn edit(get_item_response: web::Json<GetItemResponse>, token: JwToken) -> HttpResponse{
    
    println!("[+] Here is the message in the token: {}", token.message);

    let connection = &mut establish_connection();

    let results: Vec<Item> = to_do_table::table.filter(txt_title.eq(&get_item_response.title)).load(connection).unwrap();

    for result in results {
        diesel::update(to_do_table::table.find(result.id))
        .set(txt_status.eq(&get_item_response.status))
        .returning(Item::as_returning())
        .get_result(connection)
        .unwrap();
    }

    return HttpResponse::Ok().json(get_state())
}

pub async fn delete(get_item_response: web::Json<GetItemResponse>, _token: JwToken) -> HttpResponse {

    let connection = &mut establish_connection();

    let items: Vec<Item>= to_do_table::table.filter(txt_title.eq(&get_item_response.title)).order(id.asc()).load(connection).unwrap();

    let _ = diesel::delete(&items[0]).execute(connection);

   return HttpResponse::Ok().json(get_state())
}


pub fn get_state() -> SummaryItemResponse {
    let mut array_buffer: Vec<crate::enums::item_types::ItemTypes> = Vec::new();

    let mut connection: diesel::PgConnection = establish_connection();
    
    let items: Vec<Item> = to_do_table::table.order(to_do_table::columns::id.asc()).load(&mut connection).unwrap();

    for item in items {
        let status: TaskStatus = TaskStatus::from_string(item.txt_status);
        let item: ItemTypes = to_do_factory(&item.txt_title, status);
        array_buffer.push(item);
    }
    
    SummaryItemResponse::new(array_buffer)
}

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


fn process_done(item: Done, command: String, state: &Map<String, Value>) {
    let mut state = state.clone();
    match command.as_str() {
        "get" => item.get(&item.super_struct.title, &state),
        "delete" => item.delete(&item.super_struct.title, &mut state),
        "edit" => item.set_to_pending(&item.super_struct.title, &mut state),
        _ => println!("command: {} not supported", command)
    }
}


pub fn process_input(item: ItemTypes, command: String, state: &Map<String, Value>) {
    match item {
        ItemTypes::Pending(item) => process_pending(item, command, state),
        ItemTypes::Done(item) => process_done(item, command, state)
    }
}