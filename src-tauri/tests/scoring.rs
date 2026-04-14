use cadence_lib::core::scoring::{adapt_thresholds, adapt_weights, ema, score};
use cadence_lib::types::calibration::Calibration;
use cadence_lib::types::sample::FeatureVector;

#[test]
fn score_zero_features_is_zero() {
    let features = FeatureVector::default();
    let cal = Calibration::default();
    assert_eq!(score(&features, &cal.weights), 0.0);
}

#[test]
fn score_is_clamped_to_unit() {
    let features = FeatureVector {
        keys: 1.0,
        clicks: 1.0,
        moves: 1.0,
        scroll: 1.0,
        cpu: 1.0,
        process: 1.0,
        stability: 1.0,
    };
    let cal = Calibration::default();
    let s = score(&features, &cal.weights);
    assert!(s >= 0.0 && s <= 1.0);
}

#[test]
fn ema_converges_toward_score() {
    let result = ema(1.0, 0.0, 5.0, 150.0);
    assert!(result > 0.0 && result < 1.0);
}

#[test]
fn adapt_weights_preserves_normalization() {
    let cal = Calibration::default();
    let features = FeatureVector {
        keys: 0.5,
        clicks: 0.3,
        moves: 0.8,
        scroll: 0.1,
        cpu: 0.2,
        process: 0.4,
        stability: 0.9,
    };
    let new_w = adapt_weights(&cal, &features, 1.0, 0.5);
    let sum: f64 = new_w.as_array().iter().sum();
    assert!((sum - 1.0).abs() < 1e-10);
}

#[test]
fn adapt_thresholds_maintains_gap() {
    let (idle, active) = adapt_thresholds(&[0.1, 0.1, 0.1], &[0.12, 0.12, 0.12], 0.08, 0.18);
    assert!(active >= idle + 0.05);
}
