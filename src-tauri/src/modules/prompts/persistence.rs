use crate::db::Db;
use crate::modules::prompts::types::FeedbackRecord;
use rusqlite::params;

impl Db {
    pub fn insert_feedback(&self, record: &FeedbackRecord) -> Result<i64, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO feedback (timestamp, label, keys, clicks, moves, scroll, process, stability, ema) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                record.timestamp,
                record.label,
                record.keys,
                record.clicks,
                record.moves,
                record.scroll,
                record.process,
                record.stability,
                record.ema,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn get_recent_feedback(&self, limit: i64) -> Result<Vec<FeedbackRecord>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT timestamp, label, keys, clicks, moves, scroll, process, stability, ema FROM feedback ORDER BY timestamp DESC LIMIT ?1",
        )?;
        let rows = stmt.query_map(params![limit], |row| {
            Ok(FeedbackRecord {
                timestamp: row.get(0)?,
                label: row.get(1)?,
                keys: row.get(2)?,
                clicks: row.get(3)?,
                moves: row.get(4)?,
                scroll: row.get(5)?,
                process: row.get(6)?,
                stability: row.get(7)?,
                ema: row.get(8)?,
            })
        })?;
        rows.collect()
    }

    pub fn count_feedback(&self) -> Result<i64, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.query_row("SELECT COUNT(*) FROM feedback", [], |row| row.get(0))
    }
}
