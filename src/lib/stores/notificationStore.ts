import { writable } from 'svelte/store';
import type { Notification, ActivePrompt } from '../types/notification';

interface NotificationStoreState {
  notifications: Notification[];
  activePrompt: ActivePrompt | null;
}

const initialState: NotificationStoreState = {
  notifications: [],
  activePrompt: null,
};

function createNotificationStore() {
  const { subscribe, update } = writable<NotificationStoreState>(initialState);

  return {
    subscribe,
    addNotification: (notification: Notification) => 
      update(state => ({ ...state, notifications: [notification, ...state.notifications] })),
    addPrompt: (prompt: ActivePrompt) =>
      update(state => ({ ...state, activePrompt: prompt })),
    clearPrompt: () => update(state => ({ ...state, activePrompt: null }))
  };
}

export const notificationStore = createNotificationStore();
