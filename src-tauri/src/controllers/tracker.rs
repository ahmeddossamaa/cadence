use crate::services::orchestrator::Orchestrator;
use crate::types::state::TimerState;
use std::sync::Arc;

#[tauri::command]
pub fn tracker_start(
    orchestrator: tauri::State<'_, Arc<Orchestrator>>,
) -> Result<TimerState, String> {
    orchestrator.handle_user_start()?;
    orchestrator.session.build_timer_state()
}

#[tauri::command]
pub fn tracker_stop(
    orchestrator: tauri::State<'_, Arc<Orchestrator>>,
) -> Result<TimerState, String> {
    orchestrator.handle_user_stop()?;
    orchestrator.session.build_timer_state()
}

#[tauri::command]
pub fn tracker_get_status(
    orchestrator: tauri::State<'_, Arc<Orchestrator>>,
) -> Result<TimerState, String> {
    orchestrator.session.build_timer_state()
}
