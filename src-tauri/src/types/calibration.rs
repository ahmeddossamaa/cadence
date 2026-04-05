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
    pub process: f64,
    pub stability: f64,
}

impl Weights {
    pub fn as_array(&self) -> [f64; 6] {
        [
            self.keys,
            self.clicks,
            self.moves,
            self.scroll,
            self.process,
            self.stability,
        ]
    }

    pub fn from_array(arr: [f64; 6]) -> Self {
        Self {
            keys: arr[0],
            clicks: arr[1],
            moves: arr[2],
            scroll: arr[3],
            process: arr[4],
            stability: arr[5],
        }
    }
}

impl Default for Calibration {
    fn default() -> Self {
        Self {
            weights: Weights {
                keys: 0.25,
                clicks: 0.2,
                moves: 0.25,
                scroll: 0.15,
                process: 0.1,
                stability: 0.05,
            },
            idle_threshold: 0.08,
            active_threshold: 0.18,
            learning_rate: 0.1,
            samples: 0,
            updated_at: 0,
        }
    }
}
