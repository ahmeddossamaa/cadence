<script lang="ts">
  import type { Notification } from '../lib/types/notification';

  export let notification: Notification;

  function formatTime(timestamp: number): string {
    const mins = Math.floor((Date.now() - timestamp) / 60000);
    if (mins < 1) return 'Just now';
    if (mins < 60) return `${mins} min ago`;
    const h = Math.floor(mins / 60);
    return `${h}h ago`;
  }

  function getIcon(type: string): string {
    if (type === 'info') return '✓';
    if (type === 'prompt') return '?';
    if (type === 'warning') return '⚠';
    return '•';
  }

  function getIconColor(type: string): string {
    if (type === 'info') return 'var(--color-status-green)';
    if (type === 'prompt') return 'var(--color-status-amber)';
    if (type === 'warning') return 'var(--color-status-red)';
    return 'var(--color-text-secondary)';
  }
</script>

<div class="item-container">
  <div class="icon font-mono" style="color: {getIconColor(notification.type)}">
    {getIcon(notification.type)}
  </div>
  <div class="content">
    <div class="message">{notification.message}</div>
    {#if notification.response}
      <div class="response">→ {notification.response}</div>
    {/if}
    <div class="time">{formatTime(notification.timestamp)}</div>
  </div>
</div>

<style>
  .item-container {
    display: flex;
    gap: 0.75rem;
    padding: 0.5rem 0;
  }

  .icon {
    font-size: 1rem;
    font-weight: bold;
    margin-top: 0.125rem;
  }

  .content {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .message {
    font-size: 0.875rem;
    color: var(--color-text-primary);
  }

  .response {
    font-size: 0.875rem;
    color: var(--color-text-secondary);
  }

  .time {
    font-size: 0.75rem;
    color: var(--color-text-secondary);
  }
</style>
