use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TrackingState {
    Idle,
    Active,
    Away,
    Done,
}

impl TrackingState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Idle => "IDLE",
            Self::Active => "ACTIVE",
            Self::Away => "AWAY",
            Self::Done => "DONE",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "IDLE" => Some(Self::Idle),
            "ACTIVE" => Some(Self::Active),
            "AWAY" => Some(Self::Away),
            "DONE" => Some(Self::Done),
            _ => None,
        }
    }
}

impl std::fmt::Display for TrackingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StateTransition {
    pub from: TrackingState,
    pub to: TrackingState,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimerState {
    pub active_ticket_key: Option<String>,
    pub active_ticket_name: Option<String>,
    pub elapsed_seconds: u64,
    pub daily_total_seconds: u64,
    pub daily_target_seconds: u64,
    pub tracking_state: TrackingState,
}
