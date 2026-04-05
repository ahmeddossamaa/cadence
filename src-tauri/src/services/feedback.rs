use crate::adapters::persistence::Db;
use crate::core::scoring;
use crate::types::calibration::FeedbackRecord;
use crate::types::sample::FeatureVector;
use std::sync::Arc;

pub struct FeedbackService {
    db: Arc<Db>,
}

impl FeedbackService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub fn record(
        &self,
        label: i32,
        features: &FeatureVector,
        ema: f64,
        now: i64,
    ) -> Result<(), String> {
        let record = FeedbackRecord {
            timestamp: now,
            label,
            keys: features.keys,
            clicks: features.clicks,
            moves: features.moves,
            scroll: features.scroll,
            cpu: features.cpu,
            process: features.process,
            stability: features.stability,
            ema,
        };
        self.db
            .insert_feedback(&record)
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn adapt(&self, label: i32, features: &FeatureVector) -> Result<(), String> {
        let mut cal = self.db.get_calibration().map_err(|e| e.to_string())?;

        let current_score = scoring::score(features, &cal.weights);
        let label_f64 = if label > 0 { 1.0 } else { 0.0 };

        let new_weights = scoring::adapt_weights(&cal, features, label_f64, current_score);
        cal.weights = new_weights;
        cal.samples += 1;

        let recent = self
            .db
            .get_recent_feedback(50)
            .map_err(|e| e.to_string())?;

        let idle_scores: Vec<f64> = recent
            .iter()
            .filter(|f| f.label == 0)
            .map(|f| f.ema)
            .collect();
        let active_scores: Vec<f64> = recent
            .iter()
            .filter(|f| f.label > 0)
            .map(|f| f.ema)
            .collect();

        let (new_idle, new_active) = scoring::adapt_thresholds(
            &idle_scores,
            &active_scores,
            cal.idle_threshold,
            cal.active_threshold,
        );
        cal.idle_threshold = new_idle;
        cal.active_threshold = new_active;
        cal.updated_at = chrono::Utc::now().timestamp();

        self.db
            .update_calibration(&cal)
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
