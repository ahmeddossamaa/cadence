import { invoke } from '@tauri-apps/api/core';

export async function export_csv(args: { path?: string }): Promise<string> {
  return invoke('export_csv', args);
}
