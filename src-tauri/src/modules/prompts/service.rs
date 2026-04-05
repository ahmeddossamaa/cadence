use crate::db::Db;
use crate::core::scoring;
use crate::modules::prompts::types::FeedbackRecord;
use crate::types::prompt::Prompt;
use crate::types::sample::FeatureVector;
use crate::types::state::TrackingState;
use std::sync::{Arc, Mutex};

// ── PromptService ────────────────────────────────────────────────────────────

pub struct PromptService {
    last_prompt_at: Mutex<i64>,
    last_transition_at: Mutex<i64>,
    cooldown_secs: i64,
    debounce_secs: i64,
    timeout_secs: u64,
}

impl PromptService {
    pub fn new(cooldown_secs: i64, debounce_secs: i64, timeout_secs: u64) -> Self {
        Self {
            last_prompt_at: Mutex::new(0),
            last_transition_at: Mutex::new(0),
            cooldown_secs,
            debounce_secs,
            timeout_secs,
        }
    }

    pub fn record_transition(&self, now: i64) {
        *self.last_transition_at.lock().unwrap() = now;
    }

    pub fn should_prompt(
        &self,
        from: TrackingState,
        to: TrackingState,
        now: i64,
    ) -> Option<Prompt> {
        // Only prompt on ACTIVE → IDLE transitions
        if from != TrackingState::Active || to != TrackingState::Idle {
            return None;
        }

        let last_prompt = *self.last_prompt_at.lock().unwrap();
        if now - last_prompt < self.cooldown_secs {
            return None;
        }

        let last_transition = *self.last_transition_at.lock().unwrap();
        if now - last_transition < self.debounce_secs {
            return None;
        }

        *self.last_prompt_at.lock().unwrap() = now;

        Some(Prompt {
            id: format!("prompt_{}", now),
            message: "Were you working during the previous period?".to_string(),
            actions: vec!["Yes".to_string(), "No".to_string()],
            timeout_secs: self.timeout_secs,
        })
    }
}

// ── FeedbackService ──────────────────────────────────────────────────────────

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
