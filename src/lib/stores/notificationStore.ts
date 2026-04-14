import { writable } from 'svelte/store';
import type { Notification } from '../types/notification';

interface NotificationStoreState {
  notifications: Notification[];
}

const initialState: NotificationStoreState = {
  notifications: [],
};

function createNotificationStore() {
  const { subscribe, update } = writable<NotificationStoreState>(initialState);

  return {
    subscribe,
    addNotification: (notification: Notification) =>
      update(state => ({
        ...state,
        notifications: [notification, ...state.notifications].slice(0, 50)
      })),
    resolvePrompt: (id: string, response: string) =>
      update(state => ({
        ...state,
        notifications: state.notifications.map(n =>
          n.id === id ? { ...n, resolved: true, response } : n
        )
      })),
  };
}

export const notificationStore = createNotificationStore();
