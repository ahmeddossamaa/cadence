use crate::db::Db;
use crate::core::scoring;
use crate::core::state_machine::{self, Signal};
use crate::modules::tracking::workers::sampler::SampleBuffer;
use crate::types::sample::{FeatureVector, RawSample};
use crate::types::state::TrackingState;
use log::{debug, error, info, warn};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct EvalOutput {
    pub ema: f64,
    pub score: f64,
    pub features: FeatureVector,
    pub transition: Option<TrackingState>,
    pub raw_samples: Vec<RawSample>,
    pub timestamp: i64,
    pub is_checkpoint: bool,
    pub idle_elapsed_secs: u64,
    pub active_threshold: f64,
    pub idle_threshold: f64,
}

pub fn spawn(
    buffer: SampleBuffer,
    db: Arc<Db>,
    interval: Duration,
    stop: Arc<std::sync::atomic::AtomicBool>,
    tx: mpsc::Sender<EvalOutput>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        info!("[evaluator] started, interval={}ms", interval.as_millis());

        let mut prev_ema = 0.0_f64;
        let mut prev_app: Option<String> = None;
        let mut idle_since: Option<i64> = None;
        let mut last_checkpoint = chrono::Utc::now().timestamp();
        let mut tick: u64 = 0;

        while !stop.load(std::sync::atomic::Ordering::Relaxed) {
            thread::sleep(interval);

            let samples: Vec<RawSample> = {
                let mut buf = match buffer.lock() {
                    Ok(b) => b,
                    Err(e) => {
                        error!("[evaluator] buffer lock poisoned: {}", e);
                        continue;
                    }
                };
                buf.drain(..).collect()
            };

            if samples.is_empty() {
                debug!("[evaluator] tick={} no samples, skipping", tick);
                tick += 1;
                continue;
            }

            let now = chrono::Utc::now().timestamp();

            let (cal, settings) = {
                let cal = match db.get_calibration() {
                    Ok(c) => c,
                    Err(e) => {
                        warn!("[evaluator] failed to read calibration: {}", e);
                        continue;
                    }
                };
                let settings = match db.get_all_settings() {
                    Ok(s) => s,
                    Err(e) => {
                        warn!("[evaluator] failed to read settings: {}", e);
                        continue;
                    }
                };
                (cal, settings)
            };

            let evaluate_interval = settings.get_f64("evaluate_interval_secs").unwrap_or(5.0);
            let halflife = settings.get_f64("ema_halflife_secs").unwrap_or(150.0);
            let idle_timeout = settings.get_u64("idle_timeout_secs").unwrap_or(300);
            let checkpoint_interval = settings.get_i64("checkpoint_interval_secs").unwrap_or(300);

            let features = scoring::normalize(&samples, prev_app.as_deref());
            let current_score = scoring::score(&features, &cal.weights);
            let new_ema = scoring::ema(current_score, prev_ema, evaluate_interval, halflife);
            prev_ema = new_ema;

            prev_app = samples
                .last()
                .and_then(|s| s.foreground_app.clone());

            let current_state = match db.get_session() {
                Ok(s) => s.state,
                Err(_) => TrackingState::Idle,
            };

            if new_ema < cal.idle_threshold {
                if idle_since.is_none() {
                    idle_since = Some(now);
                }
            } else {
                idle_since = None;
            }

            let idle_elapsed = idle_since
                .map(|since| (now - since).max(0) as u64)
                .unwrap_or(0);

            let signal = Signal::EmaUpdate {
                ema: new_ema,
                idle_elapsed_secs: idle_elapsed,
            };

            let transition = state_machine::transition(
                current_state,
                signal,
                cal.idle_threshold,
                cal.active_threshold,
                idle_timeout,
            );

            let is_checkpoint = (now - last_checkpoint) >= checkpoint_interval;
            if is_checkpoint {
                last_checkpoint = now;
                info!("[evaluator] checkpoint at tick={}", tick);
            }

            debug!(
                "[evaluator] tick={} samples={} score={:.4} ema={:.4} state={} idle_elapsed={}s transition={:?}",
                tick,
                samples.len(),
                current_score,
                new_ema,
                current_state,
                idle_elapsed,
                transition,
            );

            if let Some(ref new_state) = transition {
                info!(
                    "[evaluator] transition {} → {}",
                    current_state, new_state
                );
            }

            let output = EvalOutput {
                ema: new_ema,
                score: current_score,
                features,
                transition,
                raw_samples: samples,
                timestamp: now,
                is_checkpoint,
                idle_elapsed_secs: idle_elapsed,
                active_threshold: cal.active_threshold,
                idle_threshold: cal.idle_threshold,
            };

            if tx.send(output).is_err() {
                error!("[evaluator] channel closed, stopping");
                break;
            }

            tick += 1;
        }

        info!("[evaluator] stopped after {} ticks", tick);
    })
}
