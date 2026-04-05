use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn window_hide(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }

    // On macOS, tell the system to activate the next app
    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::NSApplicationActivationPolicy;
        use cocoa::appkit::{NSApp, NSApplication};
        unsafe {
            let ns_app = NSApp();
            // Temporarily become a regular app so macOS can switch away, then go back to accessory
            ns_app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);
            ns_app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
        }
    }
}
