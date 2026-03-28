export interface Ticket {
  key: string;
  summary: string;
  status: string;
  timeLoggedToday: number; // seconds
}

export interface TicketDetail extends Ticket {
  assignee: string;
  storyPoints: number | null;
  estimate: number | null; // seconds
  components: string[];
  description: string;
}

export interface TicketStoreState {
  tickets: Ticket[];
  selectedKey: string | null;
  selectedDetail: TicketDetail | null;
  lastFetched: number | null; // timestamp
  loading: boolean;
}
