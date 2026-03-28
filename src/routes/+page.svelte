<script lang="ts">
  import { onMount } from 'svelte';
  import { handleKeydown, registerBindings } from '$lib/keybindings';
  import Timer from '../components/Timer.svelte';
  import StatusBadge from '../components/StatusBadge.svelte';
  import TicketDetail from '../components/TicketDetail.svelte';
  import NotificationSidebar from '../components/NotificationSidebar.svelte';
  import TicketQueue from '../components/TicketQueue.svelte';
  import { ticketStore } from '../lib/stores/ticketStore';
  import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
  let focusSection: 'queue' | 'detail' | 'sidebar' = 'queue';

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

  onMount(() => {
    registerBindings({
      toggleOverlay: async () => {
        const win = getCurrentWebviewWindow();
        if (await win.isVisible()) {
          win.hide();
        } else {
          win.show();
          win.setFocus();
        }
      },
      closeOverlay: async () => {
        const win = getCurrentWebviewWindow();
        win.hide();
      },
      navigateLeft: () => {
        if (focusSection !== 'queue') return;
        const tickets = $ticketStore.tickets;
        if (!tickets.length) return;
        const currentId = $ticketStore.selectedKey;
        const index = currentId ? tickets.findIndex((t: {key: string}) => t.key === currentId) : 0;
        const newIndex = index > 0 ? index - 1 : tickets.length - 1;
        ticketStore.selectTicket(tickets[newIndex].key);
      },
      navigateRight: () => {
        if (focusSection !== 'queue') return;
        const tickets = $ticketStore.tickets;
        if (!tickets.length) return;
        const currentId = $ticketStore.selectedKey;
        const index = currentId ? tickets.findIndex((t: {key: string}) => t.key === currentId) : -1;
        const newIndex = (index + 1) % tickets.length;
        ticketStore.selectTicket(tickets[newIndex].key);
      },
      selectTicket: () => {
        // Automatically fetched by reactivity block
      },
      scrollUp: () => {
        if (focusSection !== 'detail' && focusSection !== 'sidebar') return;
        
        if (focusSection === 'detail') {
          const el = document.querySelector('.description');
          if (el) el.scrollBy({ top: -40, behavior: 'smooth' });
        } else if (focusSection === 'sidebar') {
          const el = document.querySelector('.sidebar-container');
          if (el) el.scrollBy({ top: -40, behavior: 'smooth' });
        }
      },
      scrollDown: () => {
        if (focusSection !== 'detail' && focusSection !== 'sidebar') return;
        
        if (focusSection === 'detail') {
          const el = document.querySelector('.description');
          if (el) el.scrollBy({ top: 40, behavior: 'smooth' });
        } else if (focusSection === 'sidebar') {
          const el = document.querySelector('.sidebar-container');
          if (el) el.scrollBy({ top: 40, behavior: 'smooth' });
        }
      },
      cycleFocus: () => {
        if (focusSection === 'queue') focusSection = 'detail';
        else if (focusSection === 'detail') focusSection = 'sidebar';
        else focusSection = 'queue';
      },
      quickSwitch: (index) => {
        const tickets = $ticketStore.tickets;
        if (tickets[index]) {
          ticketStore.selectTicket(tickets[index].key);
        }
      },
      togglePause: () => { console.log('Toggle Pause'); },
      refreshJira: () => {
        import('../lib/ipc/jira').then(m => {
          m.jira_sync().then(tickets => ticketStore.updateList(tickets));
        });
      },
      openSettings: () => { console.log('Open Settings'); },
      acceptPrompt: () => {
        if (focusSection !== 'sidebar') return;
        console.log('Accept Prompt');
      },
      dismissPrompt: () => {
        if (focusSection !== 'sidebar') return;
        import('../lib/stores/notificationStore').then(m => m.notificationStore.clearPrompt());
      }
    });
  });
</script>

<svelte:window on:keydown={handleKeydown} />

<div class="overlay-app" class:focus-queue={focusSection === 'queue'} class:focus-detail={focusSection === 'detail'} class:focus-sidebar={focusSection === 'sidebar'}>
  <div class="main-content">
    <div class="top-bar">
      <Timer />
      <StatusBadge />
    </div>
    
    <div class="center-content">
      <TicketDetail />
    </div>

    <div class="bottom-bar">
      <TicketQueue />
    </div>
  </div>

  <NotificationSidebar />
</div>

<style>
  .overlay-app {
    display: flex;
    width: 100vw;
    height: 100vh;
    background-color: var(--color-main-window-bg);
    color: var(--color-text-primary);
    padding: 24px;
    box-sizing: border-box;
    gap: 24px;
  }

  .main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    height: 100%;
    min-width: 0;
  }

  .top-bar {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .center-content {
    flex: 1;
    overflow: hidden;
    display: flex;
    flex-direction: column;
  }

  .bottom-bar {
    flex-shrink: 0;
  }

  /* Visual indication of focused section */
  .focus-queue .bottom-bar { box-shadow: inset 0 0 0 2px var(--color-highlight); }
  .focus-detail .center-content { box-shadow: inset 0 0 0 2px var(--color-highlight); }
  .focus-sidebar :global(.sidebar-container) { box-shadow: inset 0 0 0 2px var(--color-highlight); }
</style>
