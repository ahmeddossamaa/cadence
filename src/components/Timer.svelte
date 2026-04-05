<script lang="ts">
  import { onMount } from 'svelte';
  import { timerStore } from '../lib/stores/timerStore';

  let elapsedSeconds = 0;

  function tick() {
    const { startedAt } = $timerStore;
    elapsedSeconds = startedAt != null
      ? Math.max(0, Math.floor(Date.now() / 1000) - startedAt)
      : 0;
  }

  onMount(() => {
    tick();
    const interval = setInterval(tick, 1000);
    return () => clearInterval(interval);
  });

  // Re-sync immediately when startedAt changes (state transition)
  $: $timerStore.startedAt, tick();

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

  $: elapsed = formatTime(elapsedSeconds);
  $: dailyTotal = formatHoursMinutes($timerStore.dailyTotalSeconds);
  $: dailyTarget = formatHoursMinutes($timerStore.dailyTargetSeconds);
  $: progress = Math.min(($timerStore.dailyTotalSeconds / $timerStore.dailyTargetSeconds) * 100, 100);
  $: state = $timerStore.trackingState;

  function getStatusColor(state: string) {
    if (state === 'ACTIVE') return 'var(--color-status-green)';
    if (state === 'MEETING') return 'var(--color-status-amber)';
    if (state === 'DONE') return 'var(--color-highlight)';
    return 'var(--color-text-secondary)';
  }
</script>

<div class="timer-compact">
  <div class="state-row">
    <span class="status-dot" style="background-color: {getStatusColor(state)}"></span>
    <span class="state-label">{state}</span>
  </div>
  <div class="elapsed font-mono">{elapsed}</div>
  <div class="daily font-mono">{dailyTotal} / {dailyTarget}</div>
  <div class="progress-bar">
    <div class="progress-fill" style="width: {progress}%"></div>
  </div>
</div>

<style>
  .timer-compact {
    display: flex;
    flex-direction: column;
    gap: 0.375rem;
    min-width: 160px;
    justify-content: center;
  }

  .state-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .status-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .state-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--color-text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .elapsed {
    font-size: 1.75rem;
    font-weight: 500;
    color: var(--color-text-primary);
    line-height: 1;
  }

  .daily {
    font-size: 0.8125rem;
    color: var(--color-text-secondary);
  }

  .progress-bar {
    height: 3px;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 2px;
    overflow: hidden;
    margin-top: 0.125rem;
  }

  .progress-fill {
    height: 100%;
    background: var(--color-status-green);
    transition: width 1s linear;
  }
</style>
