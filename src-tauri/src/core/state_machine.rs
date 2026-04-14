use crate::types::state::TrackingState;

#[derive(Debug, Clone, Copy)]
pub enum Signal {
    EmaUpdate { ema: f64, idle_elapsed_secs: u64 },
    ScreenLocked,
    ScreenUnlocked,
    UserDone,
}

pub fn transition(
    current: TrackingState,
    signal: Signal,
    idle_threshold: f64,
    active_threshold: f64,
    idle_timeout_secs: u64,
) -> Option<TrackingState> {
    match (current, signal) {
        // Screen lock always transitions to AWAY (unless already DONE)
        (TrackingState::Done, Signal::ScreenLocked) => None,
        (_, Signal::ScreenLocked) => Some(TrackingState::Away),

        // Screen unlock goes to IDLE, let evaluator detect activity
        (TrackingState::Away, Signal::ScreenUnlocked) => Some(TrackingState::Idle),
        (_, Signal::ScreenUnlocked) => None,

        // User explicitly marks done
        (_, Signal::UserDone) => Some(TrackingState::Done),

        // DONE is a terminal state for EMA signals
        (TrackingState::Done, Signal::EmaUpdate { .. }) => None,

        // AWAY ignores EMA (waiting for screen unlock)
        (TrackingState::Away, Signal::EmaUpdate { .. }) => None,

        // IDLE + high EMA → ACTIVE
        (TrackingState::Idle, Signal::EmaUpdate { ema, .. }) if ema > active_threshold => {
            Some(TrackingState::Active)
        }

        // ACTIVE + low EMA for long enough → IDLE
        (TrackingState::Active, Signal::EmaUpdate { ema, idle_elapsed_secs })
            if ema < idle_threshold && idle_elapsed_secs >= idle_timeout_secs =>
        {
            Some(TrackingState::Idle)
        }

        // Hysteresis: no change
        _ => None,
    }
}