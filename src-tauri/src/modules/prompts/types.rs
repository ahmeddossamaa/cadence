use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct FeedbackRecord {
    pub timestamp: i64,
    pub label: i32,
    pub keys: f64,
    pub clicks: f64,
    pub moves: f64,
    pub scroll: f64,
    pub process: f64,
    pub stability: f64,
    pub ema: f64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PromptResponse {
    pub id: String,
    pub value: i32,
}
