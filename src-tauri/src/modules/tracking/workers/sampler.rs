use crate::platform::PlatformSampler;
use crate::types::sample::RawSample;
use log::{debug, error, info};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub type SampleBuffer = Arc<Mutex<VecDeque<RawSample>>>;

pub fn new_buffer() -> SampleBuffer {
    Arc::new(Mutex::new(VecDeque::with_capacity(64)))
}

pub fn spawn(
    sampler: Box<dyn PlatformSampler>,
    buffer: SampleBuffer,
    interval: Duration,
    stop: Arc<std::sync::atomic::AtomicBool>,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        info!("[sampler] started, interval={}ms", interval.as_millis());

        let mut tick: u64 = 0;
        while !stop.load(std::sync::atomic::Ordering::Relaxed) {
            let sample = sampler.sample();

            debug!(
                "[sampler] tick={} keys={} clicks={} moves={} scroll={} cpu={:.1} app={:?}",
                tick, sample.keys, sample.clicks, sample.moves, sample.scroll, sample.cpu, sample.foreground_app
            );

            match buffer.lock() {
                Ok(mut buf) => buf.push_back(sample),
                Err(e) => error!("[sampler] buffer lock poisoned: {}", e),
            }

            tick += 1;
            thread::sleep(interval);
        }

        info!("[sampler] stopped after {} ticks", tick);
    })
}
