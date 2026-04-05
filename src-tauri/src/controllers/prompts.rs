use crate::services::orchestrator::Orchestrator;
use crate::types::prompt::PromptResponse;
use std::sync::Arc;

#[tauri::command]
pub fn prompt_respond(
    orchestrator: tauri::State<'_, Arc<Orchestrator>>,
    response: PromptResponse,
) -> Result<(), String> {
    orchestrator.handle_prompt_response(response.value)
}
