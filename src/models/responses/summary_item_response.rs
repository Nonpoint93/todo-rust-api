use actix_web::{body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder};
use serde::Serialize;

use crate::{enums::item_types::ItemTypes, models::base::Base};

#[derive(Serialize)]
pub struct SummaryItemResponse{
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8
}


impl SummaryItemResponse{
    pub fn new(input_items: Vec<ItemTypes>) -> SummaryItemResponse{
        let mut pending_array_buffer = Vec::new();
        let mut done_array_buffer = Vec::new();

        for item in input_items{
            match item{
                ItemTypes::Pending(packed) => pending_array_buffer.push(packed.super_struct),
                ItemTypes::Done(packed) => done_array_buffer.push(packed.super_struct)
            }
        }

        let done_count: i8 = done_array_buffer.len() as i8;
        let pending_count: i8 = pending_array_buffer.len() as i8;

        SummaryItemResponse{
            pending_items: pending_array_buffer,
            done_item_count: done_count,
            pending_item_count: pending_count,
            done_items: done_array_buffer
        }
    }
}


impl Responder for SummaryItemResponse{
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok().content_type(ContentType::json())
        .body(body)
    }
}