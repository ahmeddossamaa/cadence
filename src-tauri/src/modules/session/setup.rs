use crate::db::Db;
use crate::types::state::TrackingState;
use std::sync::Arc;

/// If the app crashed while ACTIVE, close the stale block so we start clean.
pub fn recover(db: &Arc<Db>) {
    if let Ok(session) = db.get_session() {
        if session.state == TrackingState::Active {
            log::warn!(
                "[startup] recovering stale ACTIVE session, block_id={:?}",
                session.block_id
            );
            if let Some(block_id) = session.block_id {
                let _ = db.close_block(block_id, session.checkpoint_at);
            }
            let now = chrono::Utc::now().timestamp();
            let _ = db.update_session_state(TrackingState::Idle, None);
            let _ = db.update_session_checkpoint(now);
        }
    }
}
