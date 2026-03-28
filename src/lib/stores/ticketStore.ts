import { writable } from 'svelte/store';
import type { Ticket, TicketDetail, TicketStoreState } from '../types/ticket';

const initialState: TicketStoreState = {
  tickets: [],
  selectedKey: null,
  selectedDetail: null,
  lastFetched: null,
  loading: false,
};

function createTicketStore() {
  const { subscribe, set, update } = writable<TicketStoreState>(initialState);

  return {
    subscribe,
    updateList: (tickets: Ticket[]) => update(state => ({ ...state, tickets, lastFetched: Date.now() })),
    selectTicket: (key: string | null) => update(state => ({ ...state, selectedKey: key, selectedDetail: null })),
    setDetail: (detail: TicketDetail) => update(state => ({ ...state, selectedDetail: detail }))
  };
}

export const ticketStore = createTicketStore();
