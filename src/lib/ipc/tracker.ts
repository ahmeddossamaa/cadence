import { invoke } from '@tauri-apps/api/core';
import type { TimerState } from '../types/timer';

export async function tracker_start(): Promise<TimerState> {
  return invoke('tracker_start');
}

export async function tracker_stop(): Promise<TimerState> {
  return invoke('tracker_stop');
}

export async function tracker_get_status(): Promise<TimerState> {
  return invoke('tracker_get_status');
}
