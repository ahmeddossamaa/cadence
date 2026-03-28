<script lang="ts">
  import { notificationStore } from '../lib/stores/notificationStore';
  import NotificationItem from './NotificationItem.svelte';
  import Prompt from './Prompt.svelte';

  $: notifications = $notificationStore.notifications;
  $: activePrompt = $notificationStore.activePrompt;
</script>

<div class="sidebar-container">
  {#if activePrompt}
    <div class="prompt-section">
      <Prompt prompt={activePrompt} />
    </div>
  {/if}

  <div class="history-section">
    {#each notifications as notification, index (notification.id)}
      <div class="notification-wrapper">
        <NotificationItem {notification} />
        {#if index < notifications.length - 1}
          <hr class="divider" />
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  .sidebar-container {
    width: 320px;
    background: rgba(20, 45, 68, 0.95); /* Darker sidebar */
    display: flex;
    flex-direction: column;
    padding: 1.5rem;
    gap: 2rem;
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

  .prompt-section {
    flex-shrink: 0;
  }

  .history-section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
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
