use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn window_hide(app: AppHandle) {
    if let Some(flag) = app.try_state::<Arc<AtomicBool>>() {
        flag.store(false, Ordering::SeqCst);
    }
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }

    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::NSApplicationActivationPolicy;
        use cocoa::appkit::{NSApp, NSApplication};
        unsafe {
            let ns_app = NSApp();
            ns_app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);
            ns_app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
        }
    }
}
