use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestContract {
    pub id: Uuid,
    pub message: String,
}

impl TestContract {
    pub fn new() -> Self {
        Self {
            id: Uuid::now_v7(),
            message: "This is a test message".into(),
        }
    }
}
