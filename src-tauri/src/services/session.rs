use crate::adapters::persistence::Db;
use crate::adapters::persistence::session::SessionRow;
use crate::types::state::{TimerState, TrackingState};
use std::sync::Arc;

pub struct SessionService {
    db: Arc<Db>,
}

impl SessionService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub fn get(&self) -> Result<SessionRow, String> {
        self.db.get_session().map_err(|e| e.to_string())
    }

    pub fn update_state(
        &self,
        state: TrackingState,
        block_id: Option<i64>,
    ) -> Result<(), String> {
        self.db
            .update_session_state(state, block_id)
            .map_err(|e| e.to_string())
    }

    pub fn checkpoint(&self, now: i64) -> Result<(), String> {
        self.db
            .update_session_checkpoint(now)
            .map_err(|e| e.to_string())
    }

    pub fn add_untagged(&self, secs: i64) -> Result<(), String> {
        let session = self.get()?;
        self.db
            .update_session_untagged(session.untagged_secs + secs)
            .map_err(|e| e.to_string())
    }

    pub fn clear_untagged(&self) -> Result<(), String> {
        self.db
            .update_session_untagged(0)
            .map_err(|e| e.to_string())
    }

    pub fn build_timer_state(&self) -> Result<TimerState, String> {
        let session = self.get()?;
        let now = chrono::Utc::now().timestamp();

        let elapsed = if session.state == TrackingState::Active {
            (now - session.started_at).max(0) as u64
        } else {
            0
        };

        let today_start = today_midnight();
        let blocks = self
            .db
            .get_blocks_since(today_start)
            .map_err(|e| e.to_string())?;

        let daily_total: u64 = blocks
            .iter()
            .filter(|b| b.state == "ACTIVE")
            .map(|b| {
                let end = b.ended_at.unwrap_or(now);
                (end - b.started_at).max(0) as u64
            })
            .sum();

        let settings = self.db.get_all_settings().map_err(|e| e.to_string())?;
        let daily_target = settings.get_u64("daily_target_secs").unwrap_or(28800);

        Ok(TimerState {
            active_ticket_key: None,
            active_ticket_name: None,
            elapsed_seconds: elapsed,
            daily_total_seconds: daily_total,
            daily_target_seconds: daily_target,
            tracking_state: session.state,
        })
    }
}

fn today_midnight() -> i64 {
    let now = chrono::Utc::now();
    now.date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp()
}
