export interface Features {
  keys: number;
  clicks: number;
  moves: number;
  scroll: number;
  process: number;
  stability: number;
}

export interface Diagnostics {
  ema: number;
  score: number;
  features: Features;
  state: string;
  idleElapsedSecs: number;
  activeThreshold: number;
  idleThreshold: number;
}

export interface TransitionEntry {
  from: string;
  to: string;
  timestamp: number;
}
