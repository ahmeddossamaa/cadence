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
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.08);
    padding: 0.875rem 1.125rem;
    min-width: 140px;
    border-radius: 8px;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    transition: all 150ms ease;
    cursor: pointer;
    user-select: none;
  }

  .card:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .card.selected {
    background: rgba(255, 255, 255, 0.1);
  }

  .card.active {
    border-color: rgba(255, 255, 255, 0.18);
    background: rgba(255, 255, 255, 0.1);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
    min-width: 170px;
    padding: 1rem 1.25rem;
  }

  .card.active .key {
    font-size: 1.125rem;
  }

  .card.active .time {
    font-size: 0.9375rem;
  }

  .key {
    font-size: 0.9375rem;
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .time {
    font-size: 0.8125rem;
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
