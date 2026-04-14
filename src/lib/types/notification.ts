export interface Notification {
  id: string;
  type: 'info' | 'warning' | 'prompt';
  message: string;
  timestamp: number;
  resolved: boolean;
  response?: string;
}
