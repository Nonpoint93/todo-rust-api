use serde::ser::{Serialize, SerializeStruct, Serializer};

#[derive(Clone)]
pub enum TaskStatus {
    DONE,
    PENDING
}

impl TaskStatus {

    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => {"DONE".to_string()},
            &Self::PENDING => {"PENDING".to_string()}
        }
    }

    pub fn from_string(input_string: String) -> Self {
        match input_string.as_str() {
        "DONE" => TaskStatus::DONE,
        "PENDING" => TaskStatus::PENDING,
        _ => panic!("input {} not supported",
        input_string)
        }
    }
}


impl Serialize for TaskStatus {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("TaskStatus", 1)?;
        s.serialize_field("status", &self.stringify())?;
        s.end()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status_stringify() {
        let pending = TaskStatus::PENDING;
        let done = TaskStatus::DONE;

        assert_eq!(pending.stringify(), "PENDING");
        assert_eq!(done.stringify(), "DONE");
    }

    #[test]
    fn test_task_status_from_string_valid() {
        assert!(matches!(TaskStatus::from_string("PENDING".to_string()), TaskStatus::PENDING));
        assert!(matches!(TaskStatus::from_string("DONE".to_string()), TaskStatus::DONE));
    }

    #[test]
    #[should_panic(expected = "input INVALID not supported")]
    fn test_task_status_from_string_invalid_panics() {
        TaskStatus::from_string("INVALID".to_string());
    }
}