use crate::db::Db;
use crate::core::state_machine::{self, Signal};
use crate::types::diagnostics::Diagnostics;
use crate::modules::notifications::NotificationService;
use crate::modules::prompts::service::{FeedbackService, PromptService};
use crate::modules::session::SessionService;
use crate::modules::tracking::workers::evaluator::EvalOutput;
use crate::modules::tracking::types::{BlockSource, NewBlock};
use crate::types::sample::RawSample;
use crate::types::state::{StateTransition, TrackingState};
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};

// ── BlockService ─────────────────────────────────────────────────────────────

pub struct BlockService {
    db: Arc<Db>,
}

impl BlockService {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub fn open(
        &self,
        state: TrackingState,
        now: i64,
        source: BlockSource,
        parent_id: Option<i64>,
    ) -> Result<i64, String> {
        let block = NewBlock {
            state: state.as_str().to_string(),
            started_at: now,
            source,
            parent_id,
        };
        self.db.insert_block(&block).map_err(|e| e.to_string())
    }

    pub fn close(&self, block_id: i64, now: i64) -> Result<(), String> {
        self.db
            .close_block(block_id, now)
            .map_err(|e| e.to_string())
    }

    pub fn checkpoint(
        &self,
        block_id: i64,
        samples: &[RawSample],
        ema: f64,
        now: i64,
    ) -> Result<(), String> {
        let (keys, clicks, moves, scroll) = aggregate_samples(samples);
        let (app_switches, dominant_app) = compute_app_stats(samples);

        self.db
            .update_block_metrics(
                block_id,
                keys,
                clicks,
                moves,
                scroll,
                ema,
                app_switches,
                dominant_app.as_deref(),
                now,
            )
            .map_err(|e| e.to_string())
    }

    pub fn tag_gap(
        &self,
        state: TrackingState,
        started_at: i64,
        ended_at: i64,
        parent_id: Option<i64>,
    ) -> Result<i64, String> {
        let block_id = self.open(state, started_at, BlockSource::User, parent_id)?;
        self.close(block_id, ended_at)?;
        Ok(block_id)
    }
}

fn aggregate_samples(samples: &[RawSample]) -> (i64, i64, i64, i64) {
    let keys: i64 = samples.iter().map(|s| s.keys as i64).sum();
    let clicks: i64 = samples.iter().map(|s| s.clicks as i64).sum();
    let moves: i64 = samples.iter().map(|s| s.moves as i64).sum();
    let scroll: i64 = samples.iter().map(|s| s.scroll as i64).sum();
    (keys, clicks, moves, scroll)
}

fn compute_app_stats(samples: &[RawSample]) -> (i64, Option<String>) {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    let mut prev: Option<&str> = None;
    let mut switches: i64 = 0;

    for sample in samples {
        if let Some(app) = sample.foreground_app.as_deref() {
            *counts.entry(app).or_default() += 1;
            if let Some(p) = prev {
                if p != app {
                    switches += 1;
                }
            }
            prev = Some(app);
        }
    }

    let dominant = counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(app, _)| app.to_string());

    (switches, dominant)
}

// ── Orchestrator ─────────────────────────────────────────────────────────────

pub struct Orchestrator {
    pub session: SessionService,
    pub block: BlockService,
    pub prompt: PromptService,
    pub notification: NotificationService,
    pub feedback: FeedbackService,
    db: Arc<Db>,
    last_features: Mutex<crate::types::sample::FeatureVector>,
    last_ema: Mutex<f64>,
}

impl Orchestrator {
    pub fn new(
        db: Arc<Db>,
        notification: NotificationService,
        prompt_cooldown: i64,
        prompt_debounce: i64,
        prompt_timeout: u64,
    ) -> Self {
        Self {
            session: SessionService::new(db.clone()),
            block: BlockService::new(db.clone()),
            prompt: PromptService::new(prompt_cooldown, prompt_debounce, prompt_timeout),
            notification,
            feedback: FeedbackService::new(db.clone()),
            db,
            last_features: Mutex::new(crate::types::sample::FeatureVector::default()),
            last_ema: Mutex::new(0.0),
        }
    }

    pub fn handle_eval(&self, output: EvalOutput) {
        *self.last_features.lock().unwrap() = output.features.clone();
        *self.last_ema.lock().unwrap() = output.ema;

        if let Some(new_state) = output.transition {
            let session = match self.session.get() {
                Ok(s) => s,
                Err(_) => return,
            };
            let old_state = session.state;
            let _ = self.apply_transition(old_state, new_state, output.timestamp);
        }

        if output.is_checkpoint {
            let _ = self.handle_checkpoint(&output);
        }

        let _ = self.emit_timer_tick();

        let state_str = self.session.get()
            .map(|s| s.state.as_str().to_string())
            .unwrap_or_default();
        self.notification.emit_diagnostics(&Diagnostics {
            ema: output.ema,
            score: output.score,
            features: output.features,
            state: state_str,
            idle_elapsed_secs: output.idle_elapsed_secs,
            active_threshold: output.active_threshold,
            idle_threshold: output.idle_threshold,
        });
    }

    pub fn handle_screen_lock(&self) {
        let now = chrono::Utc::now().timestamp();
        let session = match self.session.get() {
            Ok(s) => s,
            Err(_) => return,
        };

        let cal = match self.db.get_calibration() {
            Ok(c) => c,
            Err(_) => return,
        };

        if let Some(new_state) = state_machine::transition(
            session.state,
            Signal::ScreenLocked,
            cal.idle_threshold,
            cal.active_threshold,
            300,
        ) {
            let _ = self.apply_transition(session.state, new_state, now);
        }
    }

    pub fn handle_screen_unlock(&self) {
        let now = chrono::Utc::now().timestamp();
        let session = match self.session.get() {
            Ok(s) => s,
            Err(_) => return,
        };

        let cal = match self.db.get_calibration() {
            Ok(c) => c,
            Err(_) => return,
        };

        if let Some(new_state) = state_machine::transition(
            session.state,
            Signal::ScreenUnlocked,
            cal.idle_threshold,
            cal.active_threshold,
            300,
        ) {
            let _ = self.apply_transition(session.state, new_state, now);
        }
    }

    pub fn handle_user_stop(&self) -> Result<(), String> {
        let now = chrono::Utc::now().timestamp();
        let session = self.session.get()?;
        self.apply_transition(session.state, TrackingState::Done, now)
    }

    pub fn handle_user_start(&self) -> Result<(), String> {
        let now = chrono::Utc::now().timestamp();
        let session = self.session.get()?;

        if session.state == TrackingState::Done || session.state == TrackingState::Idle {
            self.apply_transition(session.state, TrackingState::Active, now)?;
        }
        Ok(())
    }

    pub fn handle_prompt_response(&self, label: i32) -> Result<(), String> {
        let now = chrono::Utc::now().timestamp();
        let features = self.last_features.lock().unwrap().clone();
        let ema = *self.last_ema.lock().unwrap();

        self.feedback.record(label, &features, ema, now)?;
        self.feedback.adapt(label, &features)?;

        let session = self.session.get()?;

        if label > 0 && session.untagged_secs > 0 {
            let gap_start = now - session.untagged_secs;
            let _ = self
                .block
                .tag_gap(TrackingState::Active, gap_start, now, session.block_id);
        }

        self.session.clear_untagged()?;
        Ok(())
    }

    pub fn spawn_eval_consumer(
        orchestrator: Arc<Self>,
        rx: mpsc::Receiver<EvalOutput>,
    ) -> std::thread::JoinHandle<()> {
        std::thread::spawn(move || {
            while let Ok(output) = rx.recv() {
                orchestrator.handle_eval(output);
            }
        })
    }

    fn apply_transition(
        &self,
        from: TrackingState,
        to: TrackingState,
        now: i64,
    ) -> Result<(), String> {
        if from == to {
            return Ok(());
        }

        let session = self.session.get()?;

        if let Some(block_id) = session.block_id {
            self.block.close(block_id, now)?;
        }

        let new_block_id = if to == TrackingState::Active {
            self.session.reset_start_time(now)?;
            Some(self.block.open(to, now, BlockSource::System, None)?)
        } else {
            None
        };

        if from == TrackingState::Active && to == TrackingState::Idle {
            let untagged = (now - session.checkpoint_at).max(0);
            self.session.add_untagged(untagged)?;
        }

        self.session.update_state(to, new_block_id)?;
        self.session.checkpoint(now)?;

        let transition = StateTransition {
            from,
            to,
            timestamp: now,
        };
        self.notification.emit_state_changed(&transition);
        self.prompt.record_transition(now);

        if let Some(prompt) = self.prompt.should_prompt(from, to, now) {
            self.notification.emit_prompt(&prompt);
        }

        Ok(())
    }

    fn handle_checkpoint(&self, output: &EvalOutput) -> Result<(), String> {
        let session = self.session.get()?;
        if let Some(block_id) = session.block_id {
            self.block
                .checkpoint(block_id, &output.raw_samples, output.ema, output.timestamp)?;
        }
        self.session.checkpoint(output.timestamp)?;
        Ok(())
    }

    fn emit_timer_tick(&self) -> Result<(), String> {
        let timer_state = self.session.build_timer_state()?;
        self.notification.emit_timer_tick(&timer_state);
        Ok(())
    }
}
