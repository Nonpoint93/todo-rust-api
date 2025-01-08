use super::base::Base;
use super::traits::get::Get;
use super::traits::create::Create;
use super::traits::edit::Edit;
use crate::enums::task_status::TaskStatus;
pub struct Pending {
    pub super_struct: Base
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base{
            title: input_title.to_string(),
            status: TaskStatus::PENDING
        };
        return Pending{super_struct: base}
    }
}

impl Get for Pending {}
impl Edit for Pending {}
impl Create for Pending {}