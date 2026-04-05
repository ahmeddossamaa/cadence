use crate::types::sample::RawSample;
use super::PlatformSampler;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct LinuxSampler {
    prev_keys: AtomicU64,
    prev_clicks: AtomicU64,
    prev_moves: AtomicU64,
    prev_scroll: AtomicU64,
}

impl LinuxSampler {
    pub fn new() -> Self {
        Self {
            prev_keys: AtomicU64::new(0),
            prev_clicks: AtomicU64::new(0),
            prev_moves: AtomicU64::new(0),
            prev_scroll: AtomicU64::new(0),
        }
    }

    fn read_evdev_counts(&self) -> (u64, u64, u64, u64) {
        let output = Command::new("cat")
            .arg("/proc/bus/input/devices")
            .output()
            .ok();
        // Placeholder: evdev requires elevated permissions or a helper daemon.
        // Return zeros until a proper input monitor is integrated.
        let _ = output;
        (0, 0, 0, 0)
    }

    fn get_foreground_app(&self) -> Option<String> {
        // Try xdotool (X11)
        let output = Command::new("xdotool")
            .args(["getactivewindow", "getwindowpid"])
            .output()
            .ok()?;
        if output.status.success() {
            let pid = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let comm = Command::new("ps")
                .args(["-p", &pid, "-o", "comm="])
                .output()
                .ok()?;
            if comm.status.success() {
                let name = String::from_utf8_lossy(&comm.stdout).trim().to_string();
                return if name.is_empty() { None } else { Some(name) };
            }
        }
        None
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

impl PlatformSampler for LinuxSampler {
    fn sample(&self) -> RawSample {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let (keys, clicks, moves, scroll) = self.read_evdev_counts();

        RawSample {
            keys: keys.saturating_sub(self.prev_keys.swap(keys, Ordering::Relaxed)),
            clicks: clicks.saturating_sub(self.prev_clicks.swap(clicks, Ordering::Relaxed)),
            moves: moves.saturating_sub(self.prev_moves.swap(moves, Ordering::Relaxed)),
            scroll: scroll.saturating_sub(self.prev_scroll.swap(scroll, Ordering::Relaxed)),
            cpu: self.get_cpu_usage(),
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
