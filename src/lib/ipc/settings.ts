import { invoke } from '@tauri-apps/api/core';
import type { Settings } from '../types/settings';

export async function settings_get(): Promise<Settings> {
  return invoke('settings_get');
}

export async function settings_update(settings: Settings): Promise<Settings> {
  return invoke('settings_update', { settings });
}
