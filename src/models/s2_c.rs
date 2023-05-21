use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SubmitWarAttemptS2C {
    tower_id: i32,
    error: Option<String>
}

impl SubmitWarAttemptS2C {
    pub fn new(tower_id: i32, error: Option<String>) -> SubmitWarAttemptS2C{
        SubmitWarAttemptS2C { tower_id: tower_id, error: error }
    }
}