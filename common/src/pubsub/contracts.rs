use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TestContract {
    pub message: String,
}

impl TestContract {
    pub fn new() -> Self {
        Self {
            message: "This is a test message".into(),
        }
    }
}
