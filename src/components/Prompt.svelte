<script lang="ts">
  import type { ActivePrompt } from '../lib/types/notification';
  import { prompt_respond } from '../lib/ipc/prompts';
  import { notificationStore } from '../lib/stores/notificationStore';

  export let prompt: ActivePrompt;

  function handleActionClick(actionValue: string) {
    prompt_respond({ id: prompt.id, value: actionValue }).then(() => {
      notificationStore.clearPrompt();
    });
  }
</script>

<div class="prompt-container">
  <div class="icon">?</div>
  <div class="content">
    <div class="message">{prompt.message}</div>
    <div class="actions">
      {#each prompt.actions as action, i}
        <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
        <div class="action font-mono" on:click={() => handleActionClick(action.value)}>
          <span class="key-hint">{i === 0 ? 'y' : 'n'}</span>
          {action.label}
        </div>
      {/each}
    </div>
  </div>
</div>

<style>
  .prompt-container {
    background: rgba(0, 0, 0, 0.2);
    padding: 1.25rem;
    border-radius: 8px;
    border: 1px solid var(--color-accent-dark);
    display: flex;
    gap: 1.25rem;
  }

  .icon {
    color: var(--color-status-amber);
    font-weight: bold;
    font-size: 1.25rem;
    margin-top: -0.125rem;
  }

  .content {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .message {
    font-size: 0.875rem;
    color: var(--color-text-primary);
  }

  .actions {
    display: flex;
    gap: 1rem;
    margin-top: 0.5rem;
  }

  .action {
    font-size: 0.875rem;
    background: rgba(255,255,255,0.1);
    padding: 0.5rem 1rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
    border-radius: 4px;
    cursor: pointer;
    transition: background 150ms ease;
  }

  .action:hover {
    background: rgba(255,255,255,0.2);
  }

  .key-hint {
    color: var(--color-highlight);
    font-weight: bold;
  }
</style>
