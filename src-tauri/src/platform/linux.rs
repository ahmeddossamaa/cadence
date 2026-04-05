use crate::types::sample::RawSample;
use super::PlatformSampler;
use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

// EV_KEY codes — mouse buttons start at 0x110 (272)
const BTN_MOUSE_MIN: u16 = 0x110;

// EV_REL codes
const REL_X: u16 = 0;
const REL_Y: u16 = 1;
const REL_HWHEEL: u16 = 6;
const REL_WHEEL: u16 = 8;
const REL_WHEEL_HI_RES: u16 = 11;

pub struct LinuxSampler {
    keys: Arc<AtomicU64>,
    clicks: Arc<AtomicU64>,
    moves: Arc<AtomicU64>,
    scroll: Arc<AtomicU64>,
}

impl LinuxSampler {
    pub fn new() -> Self {
        let keys = Arc::new(AtomicU64::new(0));
        let clicks = Arc::new(AtomicU64::new(0));
        let moves = Arc::new(AtomicU64::new(0));
        let scroll = Arc::new(AtomicU64::new(0));

        spawn_evdev_readers(&keys, &clicks, &moves, &scroll);

        Self { keys, clicks, moves, scroll }
    }

    fn get_foreground_app(&self) -> Option<String> {
        // xdotool works on X11 and XWayland
        let output = Command::new("xdotool")
            .args(["getactivewindow", "getwindowpid"])
            .output()
            .ok()?;
        if !output.status.success() {
            return None;
        }
        let pid = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let comm = Command::new("ps")
            .args(["-p", &pid, "-o", "comm="])
            .output()
            .ok()?;
        if comm.status.success() {
            let name = String::from_utf8_lossy(&comm.stdout).trim().to_string();
            if !name.is_empty() { Some(name) } else { None }
        } else {
            None
        }
    }

    fn get_cpu_usage(&self) -> f64 {
        let output = Command::new("ps")
            .args(["-A", "-o", "%cpu"])
            .output()
            .ok();
        match output {
            Some(out) if out.status.success() => {
                let text = String::from_utf8_lossy(&out.stdout);
                let total: f64 = text
                    .lines()
                    .skip(1)
                    .filter_map(|line| line.trim().parse::<f64>().ok())
                    .sum();
                let num_cpus = std::thread::available_parallelism()
                    .map(|n| n.get() as f64)
                    .unwrap_or(1.0);
                (total / num_cpus).min(100.0)
            }
            _ => 0.0,
        }
    }
}

/// Spawns one reader thread per `/dev/input/event*` device.
/// Requires the process user to be in the `input` group.
fn spawn_evdev_readers(
    keys: &Arc<AtomicU64>,
    clicks: &Arc<AtomicU64>,
    moves: &Arc<AtomicU64>,
    scroll: &Arc<AtomicU64>,
) {
    let entries = match fs::read_dir("/dev/input") {
        Ok(e) => e,
        Err(e) => {
            log::warn!("[linux_sampler] cannot read /dev/input: {} — input events will be zero", e);
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let fname = path.file_name().unwrap_or_default().to_string_lossy().into_owned();
        if !fname.starts_with("event") {
            continue;
        }

        let keys = keys.clone();
        let clicks = clicks.clone();
        let moves = moves.clone();
        let scroll = scroll.clone();

        thread::Builder::new()
            .name(format!("evdev-{}", fname))
            .spawn(move || {
                let mut device = match evdev::Device::open(&path) {
                    Ok(d) => d,
                    Err(e) => {
                        log::debug!("[linux_sampler] cannot open {}: {} (add user to 'input' group)", path.display(), e);
                        return;
                    }
                };
                log::debug!("[linux_sampler] monitoring {}", path.display());

                loop {
                    match device.fetch_events() {
                        Ok(events) => {
                            for ev in events {
                                handle_event(ev, &keys, &clicks, &moves, &scroll);
                            }
                        }
                        Err(e) => {
                            log::debug!("[linux_sampler] {} read error: {}", path.display(), e);
                            break;
                        }
                    }
                }
            })
            .ok();
    }
}

fn handle_event(
    ev: evdev::InputEvent,
    keys: &AtomicU64,
    clicks: &AtomicU64,
    moves: &AtomicU64,
    scroll: &AtomicU64,
) {
    use evdev::EventType;

    match ev.event_type() {
        EventType::KEY => {
            if ev.value() != 1 { return; } // ignore key-up (0) and auto-repeat (2)
            if ev.code() >= BTN_MOUSE_MIN {
                clicks.fetch_add(1, Ordering::Relaxed);
            } else {
                keys.fetch_add(1, Ordering::Relaxed);
            }
        }
        EventType::RELATIVE => {
            let delta = ev.value().unsigned_abs() as u64;
            match ev.code() {
                REL_X | REL_Y => { moves.fetch_add(delta, Ordering::Relaxed); }
                REL_WHEEL | REL_HWHEEL | REL_WHEEL_HI_RES => { scroll.fetch_add(delta, Ordering::Relaxed); }
                _ => {}
            }
        }
        _ => {}
    }
}

impl PlatformSampler for LinuxSampler {
    fn sample(&self) -> RawSample {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        // swap-to-zero atomically collects everything since the last sample
        RawSample {
            keys:   self.keys.swap(0, Ordering::Relaxed),
            clicks: self.clicks.swap(0, Ordering::Relaxed),
            moves:  self.moves.swap(0, Ordering::Relaxed),
            scroll: self.scroll.swap(0, Ordering::Relaxed),
            cpu:    self.get_cpu_usage(),
            foreground_app: self.get_foreground_app(),
            timestamp: now,
        }
    }

    fn is_screen_locked(&self) -> bool {
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
}
