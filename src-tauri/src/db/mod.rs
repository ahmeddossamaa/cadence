pub mod calibration;

use rusqlite::Connection;
use std::path::Path;
use std::sync::{Arc, Mutex};

pub struct Db {
    pub conn: Mutex<Connection>,
}

impl Db {
    pub fn open(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn run_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        conn.execute_batch(include_str!("../../migrations/V1__initial_schema.sql"))
            .or_else(|e| {
                if e.to_string().contains("already exists") {
                    Ok(())
                } else {
                    Err(e)
                }
            })?;
        Ok(())
    }

    pub fn ensure_singletons(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        let now = chrono::Utc::now().timestamp();

        conn.execute(
            "INSERT OR IGNORE INTO session (id, state, untagged_secs, checkpoint_at, started_at) VALUES (1, 'IDLE', 0, ?1, ?1)",
            [now],
        )?;

        conn.execute(
            "INSERT OR IGNORE INTO calibration (id, w_keys, w_clicks, w_moves, w_scroll, w_process, w_stability, idle_threshold, active_threshold, learning_rate, samples, updated_at) VALUES (1, 0.25, 0.2, 0.25, 0.15, 0.1, 0.05, 0.08, 0.15, 0.1, 0, ?1)",
            [now],
        )?;

        Ok(())
    }

    pub fn seed_default_settings(&self) -> Result<(), Box<dyn std::error::Error>> {
        let conn = self.conn.lock().unwrap();
        for (key, value) in crate::types::settings::DEFAULT_SETTINGS {
            conn.execute(
                "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
                rusqlite::params![key, value],
            )?;
        }
        Ok(())
    }
}

pub fn init(db_path: &Path) -> Arc<Db> {
    let db = Db::open(db_path).expect("failed to open database");
    db.run_migrations().expect("failed to run migrations");
    db.ensure_singletons().expect("failed to ensure singletons");
    db.seed_default_settings().expect("failed to seed settings");
    Arc::new(db)
}
