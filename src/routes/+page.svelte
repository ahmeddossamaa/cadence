<script lang="ts">
  import { onMount } from 'svelte';
  import { handleKeydown, registerBindings } from '$lib/keybindings';
  import Timer from '../components/Timer.svelte';
  import TicketDetail from '../components/TicketDetail.svelte';
  import TicketCard from '../components/TicketCard.svelte';
  import { ticketStore } from '../lib/stores/ticketStore';
  import { timerStore } from '../lib/stores/timerStore';
  import { invoke } from '@tauri-apps/api/core';
  import DiagnosticsPanel from '../components/DiagnosticsPanel.svelte';

  let showDiagnostics = false;

  $: activeKey = $timerStore.activeTicketKey;

  // Sort tickets: active ticket first
  $: sortedTickets = (() => {
    const tickets = [...$ticketStore.tickets];
    if (activeKey) {
      const idx = tickets.findIndex(t => t.key === activeKey);
      if (idx > 0) {
        const [active] = tickets.splice(idx, 1);
        tickets.unshift(active);
      }
    }
    return tickets;
  })();

  $: selectedKey = $ticketStore.selectedKey;

  // Fetch detail when selection changes
  $: {
    const currentId = $ticketStore.selectedKey;
    if (currentId && $ticketStore.selectedDetail?.key !== currentId) {
      import('../lib/ipc/tickets').then(m => {
        m.tickets_get_detail({ key: currentId }).then(detail => {
          ticketStore.setDetail(detail);
        });
      });
    }
  }

  function dismissOverlay() {
    invoke('window_hide');
  }

  onMount(() => {
    registerBindings({
      closeOverlay: async () => {
        // If detail panel is open, dismiss it first; otherwise hide the overlay
        if ($ticketStore.selectedKey) {
          ticketStore.selectTicket(null);
        } else {
          invoke('window_hide');
        }
      },
      navigateLeft: () => {
        if (!sortedTickets.length) return;
        const index = selectedKey ? sortedTickets.findIndex(t => t.key === selectedKey) : 0;
        const newIndex = index > 0 ? index - 1 : sortedTickets.length - 1;
        ticketStore.selectTicket(sortedTickets[newIndex].key);
      },
      navigateRight: () => {
        if (!sortedTickets.length) return;
        const index = selectedKey ? sortedTickets.findIndex(t => t.key === selectedKey) : -1;
        const newIndex = (index + 1) % sortedTickets.length;
        ticketStore.selectTicket(sortedTickets[newIndex].key);
      },
      dismissDetail: () => {
        ticketStore.selectTicket(null);
      },
      quickSwitch: (index) => {
        if (sortedTickets[index]) {
          ticketStore.selectTicket(sortedTickets[index].key);
        }
      },
      refreshJira: () => {
        import('../lib/ipc/jira').then(m => {
          m.jira_sync().then(tickets => ticketStore.updateList(tickets));
        });
      },
      toggleDiagnostics: () => { showDiagnostics = !showDiagnostics; },
    });
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="overlay">
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-static-element-interactions -->
  <div class="spacer" on:click={dismissOverlay}></div>

  <!-- Detail panel appears above the bar when a ticket is selected -->
  {#if selectedKey}
    <div class="detail-area">
      <TicketDetail />
    </div>
  {/if}

  <!-- Diagnostics panel (toggle with ` key) -->
  {#if showDiagnostics}
    <div class="diagnostics-area">
      <DiagnosticsPanel />
    </div>
  {/if}

  <!-- Bottom bar: Timer | divider | Tickets -->
  <div class="bottom-bar">
    <div class="timer-section">
      <Timer />
    </div>

    <div class="divider"></div>

    <div class="tickets-section">
      {#each sortedTickets as ticket (ticket.key)}
        <TicketCard
          {ticket}
          isActive={ticket.key === activeKey}
          isSelected={ticket.key === selectedKey}
          on:click={() => ticketStore.selectTicket(
            ticket.key === selectedKey ? null : ticket.key
          )}
        />
      {/each}
    </div>
  </div>
</div>

<style>
  .overlay {
    display: flex;
    flex-direction: column;
    width: 100vw;
    height: 100vh;
    color: var(--color-text-primary);
    padding: 24px;
    box-sizing: border-box;
  }

  .spacer {
    flex: 1;
    cursor: default;
  }

  .detail-area {
    margin-bottom: 16px;
  }

  .diagnostics-area {
    margin-bottom: 12px;
  }

  .bottom-bar {
    display: flex;
    align-items: stretch;
    gap: 0;
    background: var(--glass-bg);
    border: 1px solid var(--glass-border);
    border-radius: 12px;
    padding: 1rem;
    flex-shrink: 0;
  }

  .timer-section {
    flex-shrink: 0;
    display: flex;
    align-items: center;
    padding-right: 1rem;
  }

  .divider {
    width: 1px;
    background: rgba(255, 255, 255, 0.12);
    align-self: stretch;
    flex-shrink: 0;
  }

  .tickets-section {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    overflow-x: auto;
    padding-left: 1rem;
    flex: 1;
    min-width: 0;
  }

  .tickets-section::-webkit-scrollbar {
    display: none;
  }
</style>
