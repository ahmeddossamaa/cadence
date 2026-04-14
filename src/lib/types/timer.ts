export interface TimerState {
  activeTicketKey: string | null;
  activeTicketName: string | null;
  startedAt: number | null;  // Unix timestamp (seconds); null when not ACTIVE
  dailyTotalSeconds: number;
  dailyTargetSeconds: number;
  trackingState: 'IDLE' | 'ACTIVE' | 'AWAY' | 'MEETING' | 'DONE';
}
