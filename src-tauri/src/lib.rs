pub mod types;
pub mod platform;
pub mod adapters;
pub mod core;
pub mod workers;
pub mod services;
pub mod controllers;

use adapters::persistence::Db;
use services::notification::NotificationService;
use services::orchestrator::Orchestrator;
use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(|app, shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        let ctrl_shift_t = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyT);
                        let cmd_shift_t = Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyT);

                        if shortcut == &ctrl_shift_t || shortcut == &cmd_shift_t {
                            if let Some(window) = app.get_webview_window("main") {
                                if let Ok(is_visible) = window.is_visible() {
                                    if is_visible {
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
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                    }
                                }
                            }
                        }
                    }
                })
                .with_shortcut(Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyT))
                .unwrap()
                .with_shortcut(Shortcut::new(Some(Modifiers::SUPER | Modifiers::SHIFT), Code::KeyT))
                .unwrap()
                .build(),
        );
    }

    builder
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_log::Builder::new()
            .level(log::LevelFilter::Debug)
            .build())
        .setup(|app| {
            log::info!("[startup] initializing cadence");

            // Hide from dock on macOS — this is an overlay app
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

            let db = Db::open(&db_path).expect("failed to open database");
            db.run_migrations().expect("failed to run migrations");
            db.ensure_singletons().expect("failed to ensure singletons");
            db.seed_default_settings().expect("failed to seed settings");
            let db = Arc::new(db);
            log::info!("[startup] database ready at {:?}", db_path);

            // 2. Read settings for worker intervals and prompt config
            let settings = db.get_all_settings().expect("failed to read settings");
            let sample_interval = settings.get_u64("sample_interval_secs").unwrap_or(2);
            let evaluate_interval = settings.get_u64("evaluate_interval_secs").unwrap_or(5);
            let prompt_cooldown = settings.get_i64("prompt_cooldown_secs").unwrap_or(300);
            let prompt_debounce = settings.get_i64("prompt_debounce_secs").unwrap_or(60);
            let prompt_timeout = settings.get_u64("prompt_timeout_secs").unwrap_or(120);

            // 3. Orchestrator
            let notification_service = NotificationService::new(app.handle().clone());
            let orchestrator = Arc::new(Orchestrator::new(
                db.clone(),
                notification_service,
                prompt_cooldown,
                prompt_debounce,
                prompt_timeout,
            ));

            log::info!(
                "[startup] sample={}s evaluate={}s cooldown={}s debounce={}s timeout={}s",
                sample_interval, evaluate_interval, prompt_cooldown, prompt_debounce, prompt_timeout
            );

            // 4. Session recovery — if the app crashed while ACTIVE, close stale block
            if let Ok(session) = db.get_session() {
                if session.state == types::state::TrackingState::Active {
                    log::warn!("[startup] recovering stale ACTIVE session, block_id={:?}", session.block_id);
                    if let Some(block_id) = session.block_id {
                        let _ = db.close_block(block_id, session.checkpoint_at);
                    }
                    let now = chrono::Utc::now().timestamp();
                    let _ = db.update_session_state(types::state::TrackingState::Idle, None);
                    let _ = db.update_session_checkpoint(now);
                }
            }

            // 5. Workers
            let stop_flag = Arc::new(AtomicBool::new(false));

            // Sampler
            let platform_sampler = platform::create_sampler();
            let sample_buffer = workers::sampler::new_buffer();
            let _sampler_handle = workers::sampler::spawn(
                platform_sampler,
                sample_buffer.clone(),
                Duration::from_secs(sample_interval),
                stop_flag.clone(),
            );

            // Evaluator → channel → Orchestrator consumer
            let (eval_tx, eval_rx) = std::sync::mpsc::channel();
            let _evaluator_handle = workers::evaluator::spawn(
                sample_buffer,
                db.clone(),
                Duration::from_secs(evaluate_interval),
                stop_flag.clone(),
                eval_tx,
            );
            let _consumer_handle = Orchestrator::spawn_eval_consumer(
                orchestrator.clone(),
                eval_rx,
            );

            // Screen lock listener
            let _screen_lock_handle = platform::screen_lock::spawn_listener(
                orchestrator.clone(),
                stop_flag.clone(),
            );

            // 6. Manage state for IPC commands
            app.manage(db);
            app.manage(orchestrator);
            app.manage(stop_flag);

            log::info!("[startup] all workers spawned, cadence ready");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            controllers::tracker::tracker_start,
            controllers::tracker::tracker_stop,
            controllers::tracker::tracker_get_status,
            controllers::settings::settings_get,
            controllers::settings::settings_update,
            controllers::prompts::prompt_respond,
            controllers::export::export_csv,
            controllers::window::window_hide,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
