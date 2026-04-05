use crate::adapters::persistence::Db;
use crate::types::block::{BlockSource, NewBlock};
use crate::types::sample::RawSample;
use crate::types::state::TrackingState;
use std::collections::HashMap;
use std::sync::Arc;

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
        let (keys, clicks, moves, scroll, cpu) = aggregate_samples(samples);
        let (app_switches, dominant_app) = compute_app_stats(samples);

        self.db
            .update_block_metrics(
                block_id,
                keys,
                clicks,
                moves,
                scroll,
                cpu,
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

fn aggregate_samples(samples: &[RawSample]) -> (i64, i64, i64, i64, f64) {
    let keys: i64 = samples.iter().map(|s| s.keys as i64).sum();
    let clicks: i64 = samples.iter().map(|s| s.clicks as i64).sum();
    let moves: i64 = samples.iter().map(|s| s.moves as i64).sum();
    let scroll: i64 = samples.iter().map(|s| s.scroll as i64).sum();
    let cpu: f64 = if samples.is_empty() {
        0.0
    } else {
        samples.iter().map(|s| s.cpu).sum::<f64>() / samples.len() as f64
    };
    (keys, clicks, moves, scroll, cpu)
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
