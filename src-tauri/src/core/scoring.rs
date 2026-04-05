use crate::types::calibration::{Calibration, Weights};
use crate::types::sample::{FeatureVector, RawSample};

const KEYS_CEILING: f64 = 30.0;
const CLICKS_CEILING: f64 = 15.0;
const MOVES_CEILING: f64 = 500.0;
const SCROLL_CEILING: f64 = 20.0;

pub fn normalize(samples: &[RawSample], prev_app: Option<&str>) -> FeatureVector {
    if samples.is_empty() {
        return FeatureVector::default();
    }

    let n = samples.len() as f64;

    let avg_keys = samples.iter().map(|s| s.keys as f64).sum::<f64>() / n;
    let avg_clicks = samples.iter().map(|s| s.clicks as f64).sum::<f64>() / n;
    let avg_moves = samples.iter().map(|s| s.moves as f64).sum::<f64>() / n;
    let avg_scroll = samples.iter().map(|s| s.scroll as f64).sum::<f64>() / n;
    let avg_cpu = samples.iter().map(|s| s.cpu).sum::<f64>() / n;

    let unique_apps: std::collections::HashSet<&str> = samples
        .iter()
        .filter_map(|s| s.foreground_app.as_deref())
        .collect();
    let app_switches = if unique_apps.len() > 1 {
        unique_apps.len() as f64 - 1.0
    } else {
        0.0
    };
    let process = (app_switches / 5.0).min(1.0);

    let current_app = samples
        .last()
        .and_then(|s| s.foreground_app.as_deref());
    let stability = match (current_app, prev_app) {
        (Some(curr), Some(prev)) if curr == prev => 1.0,
        (Some(_), Some(_)) => 0.0,
        _ => 0.5,
    };

    FeatureVector {
        keys: (avg_keys / KEYS_CEILING).min(1.0),
        clicks: (avg_clicks / CLICKS_CEILING).min(1.0),
        moves: (avg_moves / MOVES_CEILING).min(1.0),
        scroll: (avg_scroll / SCROLL_CEILING).min(1.0),
        cpu: (avg_cpu / 100.0).min(1.0),
        process,
        stability,
    }
}

pub fn score(features: &FeatureVector, weights: &Weights) -> f64 {
    let x = features.as_array();
    let w = weights.as_array();
    let dot: f64 = x.iter().zip(w.iter()).map(|(xi, wi)| xi * wi).sum();
    dot.clamp(0.0, 1.0)
}

pub fn ema(current_score: f64, prev_ema: f64, evaluate_interval_secs: f64, halflife_secs: f64) -> f64 {
    let alpha = 1.0 - (-evaluate_interval_secs / halflife_secs).exp();
    alpha * current_score + (1.0 - alpha) * prev_ema
}

pub fn adapt_weights(
    cal: &Calibration,
    features: &FeatureVector,
    label: f64,
    current_score: f64,
) -> Weights {
    let error = label - current_score;
    let decay_rate = 0.1;
    let lr = cal.learning_rate * error.abs() / (1.0 + decay_rate * cal.samples as f64);

    let x = features.as_array();
    let w = cal.weights.as_array();

    let mut new_w: [f64; 7] = [0.0; 7];
    for i in 0..7 {
        new_w[i] = (w[i] + lr * error * x[i]).max(0.0);
    }

    let sum: f64 = new_w.iter().sum();
    if sum > 0.0 {
        for w in &mut new_w {
            *w /= sum;
        }
    }

    Weights::from_array(new_w)
}

pub fn adapt_thresholds(
    idle_scores: &[f64],
    active_scores: &[f64],
    current_idle: f64,
    current_active: f64,
) -> (f64, f64) {
    let new_idle = if idle_scores.len() >= 3 {
        let mean = idle_scores.iter().sum::<f64>() / idle_scores.len() as f64;
        let variance = idle_scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / idle_scores.len() as f64;
        mean + variance.sqrt()
    } else {
        current_idle
    };

    let new_active = if active_scores.len() >= 3 {
        let mean = active_scores.iter().sum::<f64>() / active_scores.len() as f64;
        let variance = active_scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / active_scores.len() as f64;
        mean - variance.sqrt()
    } else {
        current_active
    };

    let min_gap = 0.05;
    if new_active >= new_idle + min_gap {
        (new_idle, new_active)
    } else {
        let midpoint = (new_idle + new_active) / 2.0;
        (midpoint - min_gap / 2.0, midpoint + min_gap / 2.0)
    }
}