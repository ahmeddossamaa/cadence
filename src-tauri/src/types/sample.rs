use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RawSample {
    pub keys: u64,
    pub clicks: u64,
    pub moves: u64,
    pub scroll: u64,
    pub cpu: f64,
    pub foreground_app: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FeatureVector {
    pub keys: f64,
    pub clicks: f64,
    pub moves: f64,
    pub scroll: f64,
    pub cpu: f64,
    pub process: f64,
    pub stability: f64,
}

impl FeatureVector {
    pub fn as_array(&self) -> [f64; 7] {
        [
            self.keys,
            self.clicks,
            self.moves,
            self.scroll,
            self.cpu,
            self.process,
            self.stability,
        ]
    }
}
