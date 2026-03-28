import type { Ticket, TicketDetail } from '../types/ticket';
import type { TimerState } from '../types/timer';

const mockTickets: Ticket[] = [
  { key: 'TICK-123', summary: 'Setup frontend layout', status: 'In Progress', timeLoggedToday: 4800 },
  { key: 'TICK-456', summary: 'Fix payment calculation', status: 'In Progress', timeLoggedToday: 9255 },
  { key: 'TICK-789', summary: 'Update documentation', status: 'Open', timeLoggedToday: 2700 },
  { key: 'TICK-012', summary: 'Review PRs', status: 'In Review', timeLoggedToday: 0 }
];

export async function tickets_list(): Promise<Ticket[]> {
  return Promise.resolve(mockTickets);
}

export async function tickets_get_detail(args: { key: string }): Promise<TicketDetail> {
  const ticket = mockTickets.find(t => t.key === args.key) || mockTickets[0];
  return Promise.resolve({
    ...ticket,
    assignee: 'Ahmed',
    storyPoints: 3,
    estimate: 14400, // 4h
    components: ['Frontend'],
    description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit.'
  });
}

export async function tickets_set_active(args: { key: string }): Promise<TimerState> {
  const ticket = mockTickets.find(t => t.key === args.key) || mockTickets[0];
  return Promise.resolve({
    activeTicketKey: ticket.key,
    activeTicketName: ticket.summary,
    elapsedSeconds: ticket.timeLoggedToday,
    dailyTotalSeconds: 14400,
    dailyTargetSeconds: 28800,
    trackingState: 'ACTIVE'
  });
}
