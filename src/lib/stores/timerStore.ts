import { writable } from 'svelte/store';
import type { TimerState } from '../types/timer';

const initialState: TimerState = {
  activeTicketKey: null,
  activeTicketName: null,
  elapsedSeconds: 0,
  dailyTotalSeconds: 0,
  dailyTargetSeconds: 28800, // 8 hours
  trackingState: 'IDLE',
};

function createTimerStore() {
  const { subscribe, set, update } = writable<TimerState>(initialState);

  return {
    subscribe,
    // The instructions say timerStore.update(event.payload) for timer_tick
    update: (payload: TimerState) => set(payload),
    updateState: (payload: { from: string; to: string }) => 
      update(state => ({ ...state, trackingState: payload.to as TimerState['trackingState'] }))
  };
}

export const timerStore = createTimerStore();
