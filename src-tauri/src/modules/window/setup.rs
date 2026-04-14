use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

/// Registers Ctrl+Shift+Space to toggle the overlay window.
/// Uses Space instead of T to avoid conflicts with terminal new-tab shortcuts on Linux.
pub fn configure_shortcuts(builder: tauri::Builder<tauri::Wry>) -> tauri::Builder<tauri::Wry> {
    let overlay_visible = Arc::new(AtomicBool::new(true));
    let overlay_visible_clone = overlay_visible.clone();

    let plugin = tauri_plugin_global_shortcut::Builder::new()
        .with_handler(move |app, _shortcut, event| {
            if event.state() != ShortcutState::Pressed {
                return;
            }

            let Some(window) = app.get_webview_window("main") else { return; };

            if overlay_visible_clone.load(Ordering::SeqCst) {
                overlay_visible_clone.store(false, Ordering::SeqCst);
                let _ = window.hide();
                #[cfg(target_os = "macos")]
                {
                    use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};
                    unsafe {
                        let ns_app = NSApp();
                        ns_app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);
                        ns_app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
                    }
                }
            } else {
                overlay_visible_clone.store(true, Ordering::SeqCst);
                let _ = window.show();
                let _ = window.set_always_on_top(true);
                let _ = window.set_focus();
            }
        })
        .with_shortcut(Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::Space))
        .expect("failed to register Ctrl+Shift+Space shortcut")
        .build();

    builder
        .manage(overlay_visible)
        .plugin(plugin)
}
