use crate::types::sample::FeatureVector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostics {
    pub ema: f64,
    pub score: f64,
    pub features: FeatureVector,
    pub state: String,
    pub idle_elapsed_secs: u64,
    pub active_threshold: f64,
    pub idle_threshold: f64,
}
