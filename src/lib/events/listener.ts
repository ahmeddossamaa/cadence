import { listen } from '@tauri-apps/api/event';
import { get } from 'svelte/store';
import { timerStore } from '../stores/timerStore';
import { ticketStore } from '../stores/ticketStore';
import { notificationStore } from '../stores/notificationStore';
import { tickets_list } from '../ipc/tickets';

export async function initEventListeners() {
  await listen('timer_tick', (event: any) => {
    timerStore.update(event.payload);
  });

  await listen('state_changed', (event: any) => {
    timerStore.updateState(event.payload);
  });

  await listen('prompt', (event: any) => {
    notificationStore.addPrompt(event.payload);
  });

  await listen('notification', (event: any) => {
    notificationStore.addNotification(event.payload);
  });

  await listen('tickets_updated', (event: any) => {
    ticketStore.updateList(event.payload);
  });

  // --- MOCK BACKEND SIMULATION ---
  tickets_list().then(tickets => ticketStore.updateList(tickets));

  setInterval(() => {
    const state = get(timerStore);
    if (state.trackingState === 'ACTIVE') {
      timerStore.update({
        ...state,
        elapsedSeconds: state.elapsedSeconds + 1,
        dailyTotalSeconds: state.dailyTotalSeconds + 1
      });
    }
  }, 1000);

  setTimeout(() => {
    notificationStore.addNotification({
      id: Math.random().toString(),
      type: 'info',
      message: 'Synced to Jira',
      timestamp: Date.now() - 120000,
      resolved: true
    });
  }, 2000);

  setTimeout(() => {
    notificationStore.addPrompt({
      id: Math.random().toString(),
      message: 'Tag 12min gap',
      actions: [
        { label: 'Meeting', value: 'meeting' },
        { label: 'Ignore', value: 'ignore' }
      ],
      timeoutSeconds: 120
    });
  }, 5000);
}
