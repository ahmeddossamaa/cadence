pub mod core;
pub mod db;
pub mod modules;
pub mod platform;
pub mod types;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = modules::window::setup::configure_shortcuts(builder);
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_log::Builder::new()
            .level(log::LevelFilter::Debug)
            .build())
        .setup(|app| {
            log::info!("[startup] initializing cadence");

            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};
                unsafe {
                    let ns_app = NSApp();
                    ns_app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
                }
            }

            // 1. Database
            let data_dir = app.path().app_data_dir().expect("failed to resolve app data dir");
            let db_path = data_dir.join("cadence.db");
            let db = db::init(&db_path);
            log::info!("[startup] database ready at {:?}", db_path);

            // 2. Settings
            let settings = db.get_all_settings().expect("failed to read settings");
            let sample_interval   = settings.get_u64("sample_interval_secs").unwrap_or(2);
            let evaluate_interval = settings.get_u64("evaluate_interval_secs").unwrap_or(5);
            let prompt_cooldown   = settings.get_i64("prompt_cooldown_secs").unwrap_or(300);
            let prompt_debounce   = settings.get_i64("prompt_debounce_secs").unwrap_or(60);
            let prompt_timeout    = settings.get_u64("prompt_timeout_secs").unwrap_or(120);

            log::info!(
                "[startup] sample={}s evaluate={}s cooldown={}s debounce={}s timeout={}s",
                sample_interval, evaluate_interval, prompt_cooldown, prompt_debounce, prompt_timeout
            );

            // 3. Orchestrator
            let orchestrator = modules::tracking::setup::init_orchestrator(
                app.handle().clone(),
                db.clone(),
                prompt_cooldown,
                prompt_debounce,
                prompt_timeout,
            );

            // 4. Session recovery
            modules::session::setup::recover(&db);

            // 5. Workers
            let stop_flag = modules::tracking::setup::spawn_workers(
                db.clone(),
                orchestrator.clone(),
                sample_interval,
                evaluate_interval,
            );

            app.manage(db);
            app.manage(orchestrator);
            app.manage(stop_flag);

            log::info!("[startup] all workers spawned, cadence ready");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            modules::tracking::controller::tracker_start,
            modules::tracking::controller::tracker_stop,
            modules::tracking::controller::tracker_get_status,
            modules::settings::controller::settings_get,
            modules::settings::controller::settings_update,
            modules::prompts::controller::prompt_respond,
            modules::export::controller::export_csv,
            modules::window::controller::window_hide,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
