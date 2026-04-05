use crate::types::sample::RawSample;
use super::PlatformSampler;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

extern "C" {
    fn CGEventSourceCounterForEventType(
        state_id: i32,
        event_type: u32,
    ) -> u64;
}

const COMBINED_SESSION_STATE: i32 = 0;
const KEY_DOWN: u32 = 10;
const LEFT_MOUSE_DOWN: u32 = 1;
const MOUSE_MOVED: u32 = 5;
const SCROLL_WHEEL: u32 = 22;

pub struct MacOsSampler {
    prev_keys: AtomicU64,
    prev_clicks: AtomicU64,
    prev_moves: AtomicU64,
    prev_scroll: AtomicU64,
}

impl MacOsSampler {
    pub fn new() -> Self {
        Self {
            prev_keys: AtomicU64::new(unsafe {
                CGEventSourceCounterForEventType(COMBINED_SESSION_STATE, KEY_DOWN)
            }),
            prev_clicks: AtomicU64::new(unsafe {
                CGEventSourceCounterForEventType(COMBINED_SESSION_STATE, LEFT_MOUSE_DOWN)
            }),
            prev_moves: AtomicU64::new(unsafe {
                CGEventSourceCounterForEventType(COMBINED_SESSION_STATE, MOUSE_MOVED)
            }),
            prev_scroll: AtomicU64::new(unsafe {
                CGEventSourceCounterForEventType(COMBINED_SESSION_STATE, SCROLL_WHEEL)
            }),
        }
    }

    fn delta(&self, counter: &AtomicU64, current: u64) -> u64 {
        let prev = counter.swap(current, Ordering::Relaxed);
        current.saturating_sub(prev)
    }

    fn get_foreground_app(&self) -> Option<String> {
        let output = Command::new("osascript")
            .args(["-e", "tell application \"System Events\" to get name of first application process whose frontmost is true"])
            .output()
            .ok()?;
        if output.status.success() {
            let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if name.is_empty() { None } else { Some(name) }
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

impl PlatformSampler for MacOsSampler {
    fn sample(&self) -> RawSample {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        unsafe {
            let keys_now = CGEventSourceCounterForEventType(COMBINED_SESSION_STATE, KEY_DOWN);
            let clicks_now = CGEventSourceCounterForEventType(COMBINED_SESSION_STATE, LEFT_MOUSE_DOWN);
            let moves_now = CGEventSourceCounterForEventType(COMBINED_SESSION_STATE, MOUSE_MOVED);
            let scroll_now = CGEventSourceCounterForEventType(COMBINED_SESSION_STATE, SCROLL_WHEEL);

            RawSample {
                keys: self.delta(&self.prev_keys, keys_now),
                clicks: self.delta(&self.prev_clicks, clicks_now),
                moves: self.delta(&self.prev_moves, moves_now),
                scroll: self.delta(&self.prev_scroll, scroll_now),
                cpu: self.get_cpu_usage(),
                foreground_app: self.get_foreground_app(),
                timestamp: now,
            }
        }
    }

    fn is_screen_locked(&self) -> bool {
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
}
