import { invoke } from '@tauri-apps/api/core';

export async function prompt_respond(args: { id: string, value: number }): Promise<void> {
  return invoke('prompt_respond', { response: args });
}
