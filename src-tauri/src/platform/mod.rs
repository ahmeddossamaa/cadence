#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "linux")]
pub mod linux;

pub mod screen_lock;

use crate::types::sample::RawSample;

pub trait PlatformSampler: Send + Sync {
    fn sample(&self) -> RawSample;
    fn is_screen_locked(&self) -> bool;
}

pub fn create_sampler() -> Box<dyn PlatformSampler> {
    #[cfg(target_os = "macos")]
    {
        Box::new(macos::MacOsSampler::new())
    }
    #[cfg(target_os = "linux")]
    {
        Box::new(linux::LinuxSampler::new())
    }
}
