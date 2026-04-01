<script lang="ts">
  import { notificationStore } from '../lib/stores/notificationStore';
  import NotificationItem from './NotificationItem.svelte';

  $: notifications = $notificationStore.notifications;
</script>

<div class="sidebar-container">
  {#if notifications.length === 0}
    <div class="empty">No activity yet</div>
  {:else}
    {#each notifications as notification, index (notification.id)}
      <div class="notification-wrapper">
        <NotificationItem {notification} />
        {#if index < notifications.length - 1}
          <hr class="divider" />
        {/if}
      </div>
    {/each}
  {/if}
</div>

<style>
  .sidebar-container {
    width: 320px;
    background: var(--glass-bg);
    border: 1px solid var(--glass-border);
    display: flex;
    flex-direction: column;
    padding: 1.5rem;
    gap: 0.5rem;
    height: 100%;
    overflow-y: auto;
    border-radius: 12px;
  }

  .sidebar-container::-webkit-scrollbar {
    width: 6px;
  }

  .sidebar-container::-webkit-scrollbar-thumb {
    background: var(--color-accent-dark);
    border-radius: 3px;
  }

  .empty {
    color: var(--color-text-secondary);
    font-size: 0.875rem;
    text-align: center;
    padding: 2rem 0;
  }

  .notification-wrapper {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .divider {
    border: none;
    border-top: 1px solid rgba(255, 255, 255, 0.05);
    margin: 0;
    width: 100%;
  }
</style>
