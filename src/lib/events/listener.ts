import { listen } from '@tauri-apps/api/event';
import { timerStore } from '../stores/timerStore';
import { ticketStore } from '../stores/ticketStore';
import { notificationStore } from '../stores/notificationStore';
import { diagnosticsStore } from '../stores/diagnosticsStore';
import { tracker_get_status } from '../ipc/tracker';

export async function initEventListeners() {
  await listen('timer_tick', (event: any) => {
    timerStore.update(event.payload);
  });

  await listen('state_changed', (event: any) => {
    timerStore.updateState(event.payload);
    diagnosticsStore.onTransition(event.payload.from, event.payload.to, event.payload.timestamp);
    notificationStore.addNotification({
      id: `state_${Date.now()}`,
      type: 'info',
      message: `${event.payload.from} → ${event.payload.to}`,
      timestamp: event.payload.timestamp * 1000,
      resolved: true
    });
  });

  await listen('prompt', (event: any) => {
    notificationStore.addNotification({
      id: event.payload.id,
      type: 'prompt',
      message: event.payload.message,
      timestamp: Date.now(),
      resolved: false
    });
  });

  await listen('notification', (event: any) => {
    notificationStore.addNotification(event.payload);
  });

  await listen('tickets_updated', (event: any) => {
    ticketStore.updateList(event.payload);
  });

  await listen('diagnostics', (event: any) => {
    diagnosticsStore.onDiagnostics(event.payload);
  });

  // Fetch initial state from backend
  try {
    const status = await tracker_get_status();
    timerStore.update(status);
  } catch (e) {
    console.error('Failed to fetch initial status:', e);
  }

  // Fetch initial tickets
  try {
    const { jira_sync } = await import('../ipc/jira');
    const tickets = await jira_sync();
    ticketStore.updateList(tickets);
  } catch (e) {
    console.error('Failed to fetch initial tickets:', e);
  }
}
