use crate::db::Db;
use crate::types::state::TrackingState;
use rusqlite::params;

#[derive(Debug, Clone)]
pub struct SessionRow {
    pub state: TrackingState,
    pub block_id: Option<i64>,
    pub untagged_secs: i64,
    pub checkpoint_at: i64,
    pub started_at: i64,
}

impl Db {
    pub fn get_session(&self) -> Result<SessionRow, rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.query_row(
            "SELECT state, block_id, untagged_secs, checkpoint_at, started_at FROM session WHERE id = 1",
            [],
            |row| {
                let state_str: String = row.get(0)?;
                Ok(SessionRow {
                    state: TrackingState::from_str(&state_str).unwrap_or(TrackingState::Idle),
                    block_id: row.get(1)?,
                    untagged_secs: row.get(2)?,
                    checkpoint_at: row.get(3)?,
                    started_at: row.get(4)?,
                })
            },
        )
    }

    pub fn update_session_state(
        &self,
        state: TrackingState,
        block_id: Option<i64>,
    ) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE session SET state = ?1, block_id = ?2 WHERE id = 1",
            params![state.as_str(), block_id],
        )?;
        Ok(())
    }

    pub fn update_session_checkpoint(&self, checkpoint_at: i64) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE session SET checkpoint_at = ?1 WHERE id = 1",
            params![checkpoint_at],
        )?;
        Ok(())
    }

    pub fn update_session_untagged(&self, untagged_secs: i64) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE session SET untagged_secs = ?1 WHERE id = 1",
            params![untagged_secs],
        )?;
        Ok(())
    }

    pub fn update_session_started_at(&self, started_at: i64) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE session SET started_at = ?1 WHERE id = 1",
            params![started_at],
        )?;
        Ok(())
    }

    pub fn reset_session(&self, now: i64) -> Result<(), rusqlite::Error> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE session SET state = 'IDLE', block_id = NULL, untagged_secs = 0, checkpoint_at = ?1, started_at = ?1 WHERE id = 1",
            params![now],
        )?;
        Ok(())
    }
}
