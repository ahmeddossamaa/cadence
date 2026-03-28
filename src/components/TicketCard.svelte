<script lang="ts">
  import type { Ticket } from '../lib/types/ticket';

  export let ticket: Ticket;
  export let isActive: boolean = false;
  export let isSelected: boolean = false;

  function formatTime(seconds: number): string {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    return `${h}:${m.toString().padStart(2, '0')}`;
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
<div class="card" class:active={isActive} class:selected={isSelected} on:click>
  <div class="key font-mono">{ticket.key}</div>
  <div class="time font-mono">
    {formatTime(ticket.timeLoggedToday)}
    {#if isActive}
      <span class="active-dot">●</span>
    {/if}
  </div>
</div>

<style>
  .card {
    background: var(--color-bg-secondary);
    border: 2px solid transparent; 
    padding: 1.25rem 1.5rem;
    min-width: 240px;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    transition: all 150ms ease;
    cursor: pointer;
    user-select: none;
  }

  .card:hover {
    background: var(--color-accent-dark);
  }

  .card.selected {
    background: var(--color-accent-dark);
  }

  .card.active {
    border-color: var(--color-highlight);
    background: var(--color-accent-dark);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }

  .key {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .time {
    font-size: 0.875rem;
    color: var(--color-text-secondary);
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .active-dot {
    color: var(--color-status-green);
    font-size: 10px;
  }
</style>
