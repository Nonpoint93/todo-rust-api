use super::base::Base;
use crate::enums::task_status::TaskStatus;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE,
        };
        Done { super_struct: base }
    }
}
