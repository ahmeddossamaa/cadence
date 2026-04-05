use crate::services::orchestrator::Orchestrator;
use log::{debug, info};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn spawn_listener(
    orchestrator: Arc<Orchestrator>,
    stop: Arc<std::sync::atomic::AtomicBool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        info!("[screen_lock] listener started");
        let mut was_locked = false;

        while !stop.load(std::sync::atomic::Ordering::Relaxed) {
            let is_locked = check_screen_locked();

            if is_locked && !was_locked {
                info!("[screen_lock] screen locked");
                orchestrator.handle_screen_lock();
            } else if !is_locked && was_locked {
                info!("[screen_lock] screen unlocked");
                orchestrator.handle_screen_unlock();
            }

            debug!("[screen_lock] poll locked={}", is_locked);

            was_locked = is_locked;
            thread::sleep(Duration::from_secs(5));
        }

        info!("[screen_lock] listener stopped");
    })
}

#[cfg(target_os = "macos")]
fn check_screen_locked() -> bool {
    use std::process::Command;
    let output = Command::new("python3")
        .args([
            "-c",
            "import Quartz; print(Quartz.CGSessionCopyCurrentDictionary().get('CGSSessionScreenIsLocked', 0))",
        ])
        .output()
        .ok();
    match output {
        Some(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).trim() == "1"
        }
        _ => false,
    }
}

#[cfg(target_os = "linux")]
fn check_screen_locked() -> bool {
    use std::process::Command;
    let output = Command::new("loginctl")
        .args(["show-session", "self", "-p", "LockedHint", "--value"])
        .output()
        .ok();
    match output {
        Some(out) if out.status.success() => {
            String::from_utf8_lossy(&out.stdout).trim() == "yes"
        }
        _ => false,
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn check_screen_locked() -> bool {
    false
}
