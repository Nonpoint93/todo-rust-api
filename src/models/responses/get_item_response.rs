use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetItemResponse{
    pub title: String,
    pub status: String
}