use crate::types::sample::RawSample;
use super::PlatformSampler;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGEventSourceCounterForEventType(state_id: i32, event_type: u32) -> u64;
}

const COMBINED_SESSION_STATE: i32 = 0;
const KEY_DOWN: u32 = 10;
const LEFT_MOUSE_DOWN: u32 = 1;
const RIGHT_MOUSE_DOWN: u32 = 3;
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
                    + CGEventSourceCounterForEventType(COMBINED_SESSION_STATE, RIGHT_MOUSE_DOWN)
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
        // NSWorkspace.sharedWorkspace.frontmostApplication.localizedName
        // No Automation permission required, no process spawn.
        unsafe {
            use objc::runtime::Object;
            use objc::{class, msg_send, sel, sel_impl};

            let workspace: *mut Object = msg_send![class!(NSWorkspace), sharedWorkspace];
            if workspace.is_null() {
                return None;
            }
            let app: *mut Object = msg_send![workspace, frontmostApplication];
            if app.is_null() {
                return None;
            }
            let name: *mut Object = msg_send![app, localizedName];
            if name.is_null() {
                return None;
            }
            let utf8: *const std::ffi::c_char = msg_send![name, UTF8String];
            if utf8.is_null() {
                return None;
            }
            let s = std::ffi::CStr::from_ptr(utf8)
                .to_string_lossy()
                .into_owned();
            if s.is_empty() { None } else { Some(s) }
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
            let clicks_now =
                CGEventSourceCounterForEventType(COMBINED_SESSION_STATE, LEFT_MOUSE_DOWN)
                    + CGEventSourceCounterForEventType(COMBINED_SESSION_STATE, RIGHT_MOUSE_DOWN);
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
