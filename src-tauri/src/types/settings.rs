use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const DEFAULT_SETTINGS: &[(&str, &str)] = &[
    ("sample_interval_secs", "2"),
    ("evaluate_interval_secs", "5"),
    ("checkpoint_interval_secs", "300"),
    ("ema_halflife_secs", "60"),
    ("idle_timeout_secs", "300"),
    ("prompt_timeout_secs", "120"),
    ("prompt_cooldown_secs", "300"),
    ("prompt_debounce_secs", "60"),
    ("daily_target_secs", "28800"),
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub values: HashMap<String, String>,
}

impl Settings {
    pub fn get_i64(&self, key: &str) -> Option<i64> {
        self.values.get(key).and_then(|v| v.parse().ok())
    }

    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.values.get(key).and_then(|v| v.parse().ok())
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.values.get(key).and_then(|v| v.parse().ok())
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            values: DEFAULT_SETTINGS
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        }
    }
}
