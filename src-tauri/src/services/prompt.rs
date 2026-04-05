use crate::types::prompt::Prompt;
use crate::types::state::TrackingState;
use std::sync::Mutex;

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
