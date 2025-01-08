use super::base::Base;
use super::traits::get::Get;
use super::traits::delete::Delete;
use super::traits::edit::Edit;
use crate::enums::task_status::TaskStatus;

pub struct Done {
    pub super_struct: Base
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE
        };
        return Done{super_struct: base}
    }
}

impl Get for Done {}
impl Delete for Done {}
impl Edit for Done {}