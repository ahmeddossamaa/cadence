<script lang="ts">
  import { timerStore } from '../lib/stores/timerStore';
  import { ticketStore } from '../lib/stores/ticketStore';

  function formatSince(timestamp: number | null): string {
    if (!timestamp) return 'Never';
    const mins = Math.floor((Date.now() - timestamp) / 60000);
    return mins < 1 ? '<1m' : `${mins}m`;
  }

  $: state = $timerStore.trackingState;
  $: lastSync = formatSince($ticketStore.lastFetched);

  function getStatusColor(state: string) {
    if (state === 'ACTIVE') return 'var(--color-status-green)';
    if (state === 'MEETING') return 'var(--color-status-amber)';
    if (state === 'DONE') return 'var(--color-highlight)';
    return 'var(--color-text-secondary)'; // IDLE / AWAY
  }
</script>

<div class="badge-container">
  <div class="state-indicator" style="color: {getStatusColor(state)}">
    <span class="status-dot" style="background-color: {getStatusColor(state)}"></span>
    {state}
  </div>
  <div class="sync">
    Last sync: {lastSync}
  </div>
</div>

<style>
  .badge-container {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 0.5rem;
  }

  .state-indicator {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 1rem;
    font-weight: 600;
  }

  .status-dot {
    width: 12px;
    height: 12px;
    border-radius: 50%;
  }

  .sync {
    font-size: 0.875rem;
    color: var(--color-text-secondary);
  }
</style>
