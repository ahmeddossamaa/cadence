use crate::adapters::persistence::Db;
use crate::core::state_machine::{self, Signal};
use crate::services::block::BlockService;
use crate::services::feedback::FeedbackService;
use crate::services::notification::NotificationService;
use crate::services::prompt::PromptService;
use crate::services::session::SessionService;
use crate::types::block::BlockSource;
use crate::types::sample::FeatureVector;
use crate::types::state::{StateTransition, TrackingState};
use crate::workers::evaluator::EvalOutput;
use std::sync::{mpsc, Arc, Mutex};

pub struct Orchestrator {
    pub session: SessionService,
    pub block: BlockService,
    pub prompt: PromptService,
    pub notification: NotificationService,
    pub feedback: FeedbackService,
    db: Arc<Db>,
    last_features: Mutex<FeatureVector>,
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
            last_features: Mutex::new(FeatureVector::default()),
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

        // Close current block if one is open
        if let Some(block_id) = session.block_id {
            self.block.close(block_id, now)?;
        }

        // Open new block for ACTIVE state
        let new_block_id = if to == TrackingState::Active {
            Some(self.block.open(to, now, BlockSource::System, None)?)
        } else {
            None
        };

        // Track untagged idle time
        if from == TrackingState::Active && to == TrackingState::Idle {
            // Time since last checkpoint is untagged
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

        // Check if we should fire a prompt
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
