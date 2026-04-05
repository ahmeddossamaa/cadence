use crate::modules::notifications::types::Notification;
use crate::types::diagnostics::Diagnostics;
use crate::types::prompt::Prompt;
use crate::types::state::StateTransition;
use tauri::{AppHandle, Emitter};
use tauri_plugin_notification::NotificationExt;

pub struct NotificationService {
    app: AppHandle,
}

impl NotificationService {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    pub fn emit_state_changed(&self, transition: &StateTransition) {
        let _ = self.app.emit("state_changed", transition);
    }

    pub fn emit_prompt(&self, prompt: &Prompt) {
        let _ = self.app.emit("prompt", prompt);
        self.send_os_notification(&prompt.message, &prompt.id);
    }

    pub fn emit_notification(&self, notification: &Notification) {
        let _ = self.app.emit("notification", notification);
    }

    pub fn emit_diagnostics(&self, diagnostics: &Diagnostics) {
        let _ = self.app.emit("diagnostics", diagnostics);
    }

    pub fn emit_timer_tick(&self, state: &crate::types::state::TimerState) {
        let _ = self.app.emit("timer_tick", state);
    }

    fn send_os_notification(&self, body: &str, _prompt_id: &str) {
        let _ = self
            .app
            .notification()
            .builder()
            .title("Cadence")
            .body(body)
            .show();
    }
}
