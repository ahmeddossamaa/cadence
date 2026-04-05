use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    pub id: String,
    pub message: String,
    pub actions: Vec<String>,
    pub timeout_secs: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PromptResponse {
    pub id: String,
    pub value: i32,
}
