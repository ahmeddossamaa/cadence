use crate::db::Db;
use crate::modules::tracking::types::{Block, NewBlock};
use rusqlite::params;

impl Db {
    pub fn insert_block(&self, block: &NewBlock) -> Result<i64, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO blocks (state, started_at, source, parent_id) VALUES (?1, ?2, ?3, ?4)",
            params![
                block.state,
                block.started_at,
                block.source.as_str(),
                block.parent_id,
            ],
        )?;
        Ok(conn.last_insert_rowid())
    }

    pub fn close_block(&self, id: i64, ended_at: i64) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE blocks SET ended_at = ?1 WHERE id = ?2",
            params![ended_at, id],
        )?;
        Ok(())
    }

    pub fn update_block_metrics(
        &self,
        id: i64,
        keys: i64,
        clicks: i64,
        moves: i64,
        scroll: i64,
        ema: f64,
        app_switches: i64,
        dominant_app: Option<&str>,
        ended_at: i64,
    ) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE blocks SET keys = ?1, clicks = ?2, moves = ?3, scroll = ?4, ema = ?5, app_switches = ?6, dominant_app = ?7, ended_at = ?8 WHERE id = ?9",
            params![keys, clicks, moves, scroll, ema, app_switches, dominant_app, ended_at, id],
        )?;
        Ok(())
    }

    pub fn get_block(&self, id: i64) -> Result<Option<Block>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, state, started_at, ended_at, keys, clicks, moves, scroll, ema, app_switches, dominant_app, source, parent_id FROM blocks WHERE id = ?1",
        )?;
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(Block {
                id: row.get(0)?,
                state: row.get(1)?,
                started_at: row.get(2)?,
                ended_at: row.get(3)?,
                keys: row.get(4)?,
                clicks: row.get(5)?,
                moves: row.get(6)?,
                scroll: row.get(7)?,
                ema: row.get(8)?,
                app_switches: row.get(9)?,
                dominant_app: row.get(10)?,
                source: row.get(11)?,
                parent_id: row.get(12)?,
            })
        })?;
        rows.next().transpose()
    }

    pub fn get_blocks_since(&self, since: i64) -> Result<Vec<Block>, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, state, started_at, ended_at, keys, clicks, moves, scroll, ema, app_switches, dominant_app, source, parent_id FROM blocks WHERE started_at >= ?1 ORDER BY started_at",
        )?;
        let rows = stmt.query_map(params![since], |row| {
            Ok(Block {
                id: row.get(0)?,
                state: row.get(1)?,
                started_at: row.get(2)?,
                ended_at: row.get(3)?,
                keys: row.get(4)?,
                clicks: row.get(5)?,
                moves: row.get(6)?,
                scroll: row.get(7)?,
                ema: row.get(8)?,
                app_switches: row.get(9)?,
                dominant_app: row.get(10)?,
                source: row.get(11)?,
                parent_id: row.get(12)?,
            })
        })?;
        rows.collect()
    }
}
