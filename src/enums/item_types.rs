use crate::models::{done::Done, pending::Pending};
use super::task_status::TaskStatus;


pub enum ItemTypes {
    Pending(Pending),
    Done(Done)
}

pub fn to_do_factory(title: &str,
                     status: TaskStatus) -> ItemTypes {
    match status {
        TaskStatus::DONE => {
            ItemTypes::Done(Done::new(title))
        },
        TaskStatus::PENDING => {
            ItemTypes::Pending(Pending::new(title))
        }
    }
}