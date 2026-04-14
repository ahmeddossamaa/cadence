use cadence_lib::core::state_machine::{transition, Signal};
use cadence_lib::types::state::TrackingState;

#[test]
fn idle_to_active_on_high_ema() {
    let result = transition(TrackingState::Idle, Signal::EmaUpdate { ema: 0.3, idle_elapsed_secs: 0 }, 0.08, 0.18, 300);
    assert_eq!(result, Some(TrackingState::Active));
}

#[test]
fn active_stays_in_hysteresis() {
    let result = transition(TrackingState::Active, Signal::EmaUpdate { ema: 0.12, idle_elapsed_secs: 100 }, 0.08, 0.18, 300);
    assert_eq!(result, None);
}

#[test]
fn active_to_idle_after_timeout() {
    let result = transition(TrackingState::Active, Signal::EmaUpdate { ema: 0.05, idle_elapsed_secs: 300 }, 0.08, 0.18, 300);
    assert_eq!(result, Some(TrackingState::Idle));
}

#[test]
fn screen_lock_goes_away() {
    let result = transition(TrackingState::Active, Signal::ScreenLocked, 0.08, 0.18, 300);
    assert_eq!(result, Some(TrackingState::Away));
}

#[test]
fn screen_unlock_goes_idle() {
    let result = transition(TrackingState::Away, Signal::ScreenUnlocked, 0.08, 0.18, 300);
    assert_eq!(result, Some(TrackingState::Idle));
}

#[test]
fn done_is_terminal_for_ema() {
    let result = transition(TrackingState::Done, Signal::EmaUpdate { ema: 0.5, idle_elapsed_secs: 0 }, 0.08, 0.18, 300);
    assert_eq!(result, None);
}

#[test]
fn done_ignores_screen_lock() {
    let result = transition(TrackingState::Done, Signal::ScreenLocked, 0.08, 0.18, 300);
    assert_eq!(result, None);
}

#[test]
fn user_done_from_any_state() {
    assert_eq!(transition(TrackingState::Idle, Signal::UserDone, 0.08, 0.18, 300), Some(TrackingState::Done));
    assert_eq!(transition(TrackingState::Active, Signal::UserDone, 0.08, 0.18, 300), Some(TrackingState::Done));
    assert_eq!(transition(TrackingState::Away, Signal::UserDone, 0.08, 0.18, 300), Some(TrackingState::Done));
}
