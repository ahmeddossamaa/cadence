export interface Notification {
  id: string;
  type: 'info' | 'warning' | 'prompt';
  message: string;
  timestamp: number;
  resolved: boolean;
  response?: string; // user's answer if it was a prompt
}

export interface PromptAction {
  label: string;
  value: string;
}

export interface ActivePrompt {
  id: string;
  message: string;
  actions: PromptAction[];
  timeoutSeconds: number;
}
