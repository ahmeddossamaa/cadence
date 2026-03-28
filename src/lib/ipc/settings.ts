import type { Settings } from '../types/settings';

export async function settings_get(): Promise<Settings> {
  return Promise.resolve({});
}

export async function settings_update(settings: Settings): Promise<Settings> {
  return Promise.resolve(settings);
}
