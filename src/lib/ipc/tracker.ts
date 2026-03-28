import type { TimerState } from '../types/timer';

const mockState: TimerState = {
  activeTicketKey: 'TICK-456',
  activeTicketName: 'Fix payment calculation',
  elapsedSeconds: 9255, // 02:34:15
  dailyTotalSeconds: 14400, // 4 hours
  dailyTargetSeconds: 28800, // 8 hours
  trackingState: 'ACTIVE'
};

export async function tracker_start(): Promise<TimerState> {
  return Promise.resolve({ ...mockState, trackingState: 'ACTIVE' });
}

export async function tracker_stop(): Promise<TimerState> {
  return Promise.resolve({ ...mockState, trackingState: 'IDLE' });
}

export async function tracker_get_status(): Promise<TimerState> {
  return Promise.resolve(mockState);
}
