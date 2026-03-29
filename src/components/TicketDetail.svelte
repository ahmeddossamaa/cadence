<script lang="ts">
  import { ticketStore } from '../lib/stores/ticketStore';

  $: detail = $ticketStore.selectedDetail;
  $: loading = $ticketStore.loading;

  function formatTime(seconds: number | null): string {
    if (seconds === null) return 'N/A';
    const h = Math.floor(seconds / 3600);
    return `${h}h`;
  }
</script>

{#if detail}
  <div class="detail-panel">
    <div class="header">
      <span class="key font-mono">{detail.key}</span>
      <span class="summary">{detail.summary}</span>
    </div>
    <div class="meta">
      {detail.status}
      {#if detail.storyPoints !== null} &middot; {detail.storyPoints} pts{/if}
      {#if detail.estimate !== null} &middot; Est {formatTime(detail.estimate)}{/if}
      {#if detail.assignee} &middot; {detail.assignee}{/if}
      {#if detail.components.length} &middot; {detail.components.join(', ')}{/if}
    </div>
    {#if detail.description}
      <div class="description">{detail.description}</div>
    {/if}
  </div>
{:else if loading}
  <div class="detail-panel empty">Loading...</div>
{/if}

<style>
  .detail-panel {
    background: var(--glass-bg);
    backdrop-filter: blur(var(--glass-blur));
    -webkit-backdrop-filter: blur(var(--glass-blur));
    border: 1px solid var(--glass-border);
    border-radius: 10px;
    padding: 1.25rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-height: 280px;
  }

  .detail-panel.empty {
    align-items: center;
    justify-content: center;
    color: var(--color-text-secondary);
    padding: 1rem;
  }

  .header {
    font-size: 1rem;
    font-weight: 600;
    color: var(--color-text-primary);
    display: flex;
    gap: 0.5rem;
    align-items: baseline;
  }

  .key {
    color: var(--color-highlight);
    flex-shrink: 0;
  }

  .summary {
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .meta {
    font-size: 0.8125rem;
    color: var(--color-text-secondary);
  }

  .description {
    font-size: 0.8125rem;
    line-height: 1.5;
    color: var(--color-text-primary);
    opacity: 0.85;
    overflow-y: auto;
    max-height: 180px;
  }

  .description::-webkit-scrollbar {
    width: 4px;
  }

  .description::-webkit-scrollbar-thumb {
    background: var(--color-accent-dark);
    border-radius: 2px;
  }
</style>
