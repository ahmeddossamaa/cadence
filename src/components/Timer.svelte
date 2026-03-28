<script lang="ts">
  import { timerStore } from '../lib/stores/timerStore';

  function formatTime(seconds: number): string {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    const s = seconds % 60;
    return `${h.toString().padStart(2, '0')}:${m.toString().padStart(2, '0')}:${s.toString().padStart(2, '0')}`;
  }

  function formatHoursMinutes(seconds: number): string {
    const h = Math.floor(seconds / 3600);
    const m = Math.floor((seconds % 3600) / 60);
    return `${h}:${m.toString().padStart(2, '0')}`;
  }

  $: activeTicketKey = $timerStore.activeTicketKey;
  $: activeTicketName = $timerStore.activeTicketName;
  $: elapsed = formatTime($timerStore.elapsedSeconds);
  $: dailyTotal = formatHoursMinutes($timerStore.dailyTotalSeconds);
  $: dailyTarget = formatHoursMinutes($timerStore.dailyTargetSeconds);
  $: progress = Math.min(($timerStore.dailyTotalSeconds / $timerStore.dailyTargetSeconds) * 100, 100);
  
  $: state = $timerStore.trackingState;

  function getStatusColor(state: string) {
    if (state === 'ACTIVE') return 'var(--color-status-green)';
    if (state === 'MEETING') return 'var(--color-status-amber)';
    if (state === 'DONE') return 'var(--color-highlight)';
    return 'var(--color-text-secondary)'; // IDLE / AWAY
  }
</script>

<div class="timer-container">
  <div class="time-display">
    <span class="status-dot" style="background-color: {getStatusColor(state)}" title={state}></span>
    <span class="elapsed font-mono">{elapsed}</span>
    <span class="target font-mono">/ {dailyTarget}</span>
  </div>
  {#if activeTicketKey}
    <div class="ticket-info">
      <span class="key font-mono">{activeTicketKey}</span>
    </div>
  {/if}
  <div class="progress-bar">
    <div class="progress-fill" style="width: {progress}%"></div>
  </div>
</div>

<style>
  .timer-container {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .time-display {
    display: flex;
    align-items: baseline;
    gap: 0.75rem;
  }

  .elapsed {
    font-size: 3rem;
    font-weight: 500;
    color: var(--color-text-primary);
    line-height: 1;
  }

  .status-dot {
    display: inline-block;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    align-self: center;
  }

  .target {
    font-size: 1.5rem;
    color: var(--color-text-secondary);
  }

  .ticket-info {
    font-size: 1.5rem;
    font-weight: 600;
    color: var(--color-highlight);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .progress-bar {
    height: 4px;
    background: var(--color-bg-primary);
    border-radius: 2px;
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--color-status-green);
    transition: width 1s linear;
  }
</style>
