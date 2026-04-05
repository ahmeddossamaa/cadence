use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Calibration {
    pub weights: Weights,
    pub idle_threshold: f64,
    pub active_threshold: f64,
    pub learning_rate: f64,
    pub samples: i64,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weights {
    pub keys: f64,
    pub clicks: f64,
    pub moves: f64,
    pub scroll: f64,
    pub cpu: f64,
    pub process: f64,
    pub stability: f64,
}

impl Weights {
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

    pub fn from_array(arr: [f64; 7]) -> Self {
        Self {
            keys: arr[0],
            clicks: arr[1],
            moves: arr[2],
            scroll: arr[3],
            cpu: arr[4],
            process: arr[5],
            stability: arr[6],
        }
    }
}

impl Default for Calibration {
    fn default() -> Self {
        Self {
            weights: Weights {
                keys: 0.2,
                clicks: 0.15,
                moves: 0.2,
                scroll: 0.15,
                cpu: 0.1,
                process: 0.1,
                stability: 0.1,
            },
            idle_threshold: 0.08,
            active_threshold: 0.18,
            learning_rate: 0.1,
            samples: 0,
            updated_at: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FeedbackRecord {
    pub timestamp: i64,
    pub label: i32,
    pub keys: f64,
    pub clicks: f64,
    pub moves: f64,
    pub scroll: f64,
    pub cpu: f64,
    pub process: f64,
    pub stability: f64,
    pub ema: f64,
}
