export interface TimerState {
  activeTicketKey: string | null;
  activeTicketName: string | null;
  elapsedSeconds: number;
  dailyTotalSeconds: number;
  dailyTargetSeconds: number;
  trackingState: 'IDLE' | 'ACTIVE' | 'AWAY' | 'MEETING' | 'DONE';
}
