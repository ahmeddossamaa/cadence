import type { Ticket } from '../types/ticket';
import { tickets_list } from './tickets';

export async function jira_sync(): Promise<Ticket[]> {
  return tickets_list();
}
