use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockSource {
    System,
    User,
}

impl BlockSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::System => "SYSTEM",
            Self::User => "USER",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "SYSTEM" => Some(Self::System),
            "USER" => Some(Self::User),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub id: i64,
    pub state: String,
    pub started_at: i64,
    pub ended_at: Option<i64>,
    pub keys: i64,
    pub clicks: i64,
    pub moves: i64,
    pub scroll: i64,
    pub cpu: f64,
    pub ema: f64,
    pub app_switches: i64,
    pub dominant_app: Option<String>,
    pub source: String,
    pub parent_id: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct NewBlock {
    pub state: String,
    pub started_at: i64,
    pub source: BlockSource,
    pub parent_id: Option<i64>,
}
