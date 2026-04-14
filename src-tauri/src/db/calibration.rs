use super::Db;
use crate::types::calibration::{Calibration, Weights};
use rusqlite::params;

impl Db {
    pub fn get_calibration(&self) -> Result<Calibration, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT w_keys, w_clicks, w_moves, w_scroll, w_process, w_stability, idle_threshold, active_threshold, learning_rate, samples, updated_at FROM calibration WHERE id = 1",
            [],
            |row| {
                Ok(Calibration {
                    weights: Weights {
                        keys: row.get(0)?,
                        clicks: row.get(1)?,
                        moves: row.get(2)?,
                        scroll: row.get(3)?,
                        process: row.get(4)?,
                        stability: row.get(5)?,
                    },
                    idle_threshold: row.get(6)?,
                    active_threshold: row.get(7)?,
                    learning_rate: row.get(8)?,
                    samples: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            },
        )
    }

    pub fn update_calibration(&self, cal: &Calibration) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE calibration SET w_keys = ?1, w_clicks = ?2, w_moves = ?3, w_scroll = ?4, w_process = ?5, w_stability = ?6, idle_threshold = ?7, active_threshold = ?8, learning_rate = ?9, samples = ?10, updated_at = ?11 WHERE id = 1",
            params![
                cal.weights.keys,
                cal.weights.clicks,
                cal.weights.moves,
                cal.weights.scroll,
                cal.weights.process,
                cal.weights.stability,
                cal.idle_threshold,
                cal.active_threshold,
                cal.learning_rate,
                cal.samples,
                cal.updated_at,
            ],
        )?;
        Ok(())
    }
}
