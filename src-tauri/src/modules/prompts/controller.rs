use crate::modules::tracking::Orchestrator;
use crate::modules::prompts::types::PromptResponse;
use std::sync::Arc;

#[tauri::command]
pub fn prompt_respond(
    orchestrator: tauri::State<'_, Arc<Orchestrator>>,
    response: PromptResponse,
) -> Result<(), String> {
    orchestrator.handle_prompt_response(response.value)
}
