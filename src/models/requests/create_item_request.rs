//! # Create Item Request Module
//! 
//! Defines the expected JSON payload structure for creating a new to-do task

use serde::{Deserialize, Serialize};

/// Represents the incoming payload for creating a new task via HTTP POST
#[derive(Deserialize, Debug, Serialize)]
pub struct CreateItemRequest {
    pub title: String,
    pub status: String,
}