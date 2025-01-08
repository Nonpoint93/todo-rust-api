use serde::Serialize;

use crate::enums::task_status::TaskStatus;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus
}