<script lang="ts">
  import { ticketStore } from '../lib/stores/ticketStore';
  import { timerStore } from '../lib/stores/timerStore';
  import TicketCard from './TicketCard.svelte';

  $: tickets = $ticketStore.tickets;
  $: activeKey = $timerStore.activeTicketKey;
  $: selectedKey = $ticketStore.selectedKey;
</script>

<div class="queue-container">
  {#each tickets as ticket}
    <TicketCard 
      ticket={ticket} 
      isActive={ticket.key === activeKey}
      isSelected={ticket.key === selectedKey}
      on:click={() => ticketStore.selectTicket(ticket.key)}
    />
  {/each}
</div>

<style>
  .queue-container {
    display: flex;
    overflow-x: auto;
    gap: 1.5rem;
    padding: 1.5rem 0.5rem;
    background: transparent;
  }

  /* Hide scrollbar */
  .queue-container::-webkit-scrollbar {
    display: none;
  }
</style>
