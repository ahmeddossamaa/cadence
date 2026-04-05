use crate::db::Db;
use crate::modules::notifications::NotificationService;
use crate::modules::tracking::service::Orchestrator;
use crate::modules::tracking::workers;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Duration;

pub fn init_orchestrator(
    app_handle: tauri::AppHandle,
    db: Arc<Db>,
    prompt_cooldown: i64,
    prompt_debounce: i64,
    prompt_timeout: u64,
) -> Arc<Orchestrator> {
    let notification = NotificationService::new(app_handle);
    Arc::new(Orchestrator::new(
        db,
        notification,
        prompt_cooldown,
        prompt_debounce,
        prompt_timeout,
    ))
}

pub fn spawn_workers(
    db: Arc<Db>,
    orchestrator: Arc<Orchestrator>,
    sample_interval: u64,
    evaluate_interval: u64,
) -> Arc<AtomicBool> {
    let stop_flag = Arc::new(AtomicBool::new(false));

    let platform_sampler = crate::platform::create_sampler();
    let sample_buffer = workers::sampler::new_buffer();
    let _sampler_handle = workers::sampler::spawn(
        platform_sampler,
        sample_buffer.clone(),
        Duration::from_secs(sample_interval),
        stop_flag.clone(),
    );

    let (eval_tx, eval_rx) = std::sync::mpsc::channel();
    let _evaluator_handle = workers::evaluator::spawn(
        sample_buffer,
        db.clone(),
        Duration::from_secs(evaluate_interval),
        stop_flag.clone(),
        eval_tx,
    );
    let _consumer_handle = Orchestrator::spawn_eval_consumer(orchestrator.clone(), eval_rx);

    let _screen_lock_handle = crate::platform::screen_lock::spawn_listener(
        orchestrator,
        stop_flag.clone(),
    );

    stop_flag
}
