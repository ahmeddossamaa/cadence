<script lang="ts">
  import { ticketStore } from '../lib/stores/ticketStore';

  $: detail = $ticketStore.selectedDetail;
  $: loading = $ticketStore.loading;

  function formatTime(seconds: number | null): string {
    if (seconds === null) return 'N/A';
    const h = Math.floor(seconds / (3600));
    return `${h}h`;
  }
</script>

{#if detail}
  <div class="detail-container">
    <div class="header">
      <span class="key font-mono">{detail.key}:</span>
      <span class="summary">{detail.summary}</span>
    </div>
    <div class="meta">
      Status: {detail.status} | Points: {detail.storyPoints ?? 'N/A'} | Estimate: {formatTime(detail.estimate)}
    </div>
    <div class="meta">
      Assignee: {detail.assignee} | Component: {detail.components.join(', ') || 'None'}
    </div>
    <div class="description">
      Description: {detail.description}
    </div>
  </div>
{:else if loading}
  <div class="detail-container empty">
    Loading details...
  </div>
{:else}
  <div class="detail-container empty">
  </div>
{/if}

<style>
  .detail-container {
    padding: 1.5rem;
    background: transparent;
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    height: 100%;
  }

  .detail-container.empty {
    align-items: center;
    justify-content: center;
    color: var(--color-text-secondary);
  }

  .header {
    font-size: 1.125rem;
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .key {
    color: var(--color-highlight);
    margin-right: 0.25rem;
  }

  .meta {
    font-size: 0.875rem;
    color: var(--color-text-secondary);
  }

  .description {
    margin-top: 0.5rem;
    font-size: 0.875rem;
    line-height: 1.4;
    color: var(--color-text-primary);
    overflow-y: auto;
  }
</style>
