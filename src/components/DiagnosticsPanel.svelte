<script lang="ts">
  import { diagnosticsStore } from '../lib/stores/diagnosticsStore';

  $: d = $diagnosticsStore.current;
  $: history = $diagnosticsStore.emaHistory;
  $: transitions = $diagnosticsStore.transitions;

  function pct(v: number) {
    return `${(Math.min(Math.max(v, 0), 1) * 100).toFixed(1)}%`;
  }

  function fmt(v: number) {
    return v.toFixed(3);
  }

  function fmtTime(ts: number) {
    return new Date(ts * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' });
  }

  function sparklinePath(values: number[]): string {
    if (values.length < 2) return '';
    const w = 100;
    const h = 32;
    const points = values.map((v, i) => {
      const x = (i / (values.length - 1)) * w;
      const y = h - v * h;
      return `${x},${y}`;
    });
    return `M ${points.join(' L ')}`;
  }

  const featureKeys = ['keys', 'clicks', 'moves', 'scroll', 'process', 'stability'] as const;
</script>

<div class="panel">
  <div class="panel-header">
    <span class="panel-title">DIAGNOSTICS</span>
    <span class="hint">` to close</span>
    {#if d}
      <span class="state-badge state-{d.state.toLowerCase()}">{d.state}</span>
      {#if d.idleElapsedSecs > 0}
        <span class="idle-elapsed">idle {d.idleElapsedSecs}s</span>
      {/if}
    {/if}
  </div>

  {#if d}
    <div class="metrics">
      <!-- EMA row -->
      <div class="metric-row">
        <span class="metric-label">EMA</span>
        <div class="bar-track">
          <div class="bar-fill ema" style="width: {pct(d.ema)}"></div>
          <!-- idle threshold: ACTIVE→IDLE trigger -->
          <div class="threshold idle-thresh" style="left: {pct(d.idleThreshold)}" title="ACTIVE→IDLE below this">
            <span class="thresh-pip-label">idle</span>
          </div>
          <!-- active threshold: IDLE→ACTIVE trigger -->
          <div class="threshold active-thresh" style="left: {pct(d.activeThreshold)}" title="IDLE→ACTIVE above this">
            <span class="thresh-pip-label">go</span>
          </div>
        </div>
        <span class="metric-value">{fmt(d.ema)}</span>
      </div>
      <div class="threshold-legend">
        <span class="thresh-tag idle">▼{fmt(d.idleThreshold)} active→idle</span>
        <span class="thresh-tag active">▲{fmt(d.activeThreshold)} idle→active</span>
      </div>

      <!-- Score row -->
      <div class="metric-row">
        <span class="metric-label">Score</span>
        <div class="bar-track">
          <div class="bar-fill score" style="width: {pct(d.score)}"></div>
        </div>
        <span class="metric-value">{fmt(d.score)}</span>
      </div>
    </div>

    <!-- Sparkline -->
    {#if history.length > 1}
      <div class="sparkline-wrap">
        <svg viewBox="0 0 100 32" preserveAspectRatio="none" class="sparkline">
          <!-- threshold lines -->
          <line x1="0" y1={32 - d.activeThreshold * 32} x2="100" y2={32 - d.activeThreshold * 32}
            stroke="var(--color-status-green)" stroke-width="0.5" stroke-dasharray="2,2" />
          <line x1="0" y1={32 - d.idleThreshold * 32} x2="100" y2={32 - d.idleThreshold * 32}
            stroke="var(--color-status-amber)" stroke-width="0.5" stroke-dasharray="2,2" />
          <path d={sparklinePath(history)} fill="none" stroke="var(--color-accent)" stroke-width="1.5" />
        </svg>
        <span class="sparkline-label">EMA ({history.length} ticks)</span>
      </div>
    {/if}

    <!-- Features grid -->
    <div class="features">
      {#each featureKeys as key}
        <div class="feature">
          <span class="feature-label">{key}</span>
          <div class="feature-track">
            <div class="feature-fill" style="width: {pct(d.features[key])}"></div>
          </div>
          <span class="feature-value">{fmt(d.features[key])}</span>
        </div>
      {/each}
    </div>
  {:else}
    <p class="no-data">Waiting for first evaluator tick…</p>
  {/if}

  <!-- Transition log -->
  {#if transitions.length > 0}
    <div class="transitions">
      <span class="section-label">Transitions</span>
      {#each transitions as t}
        <div class="transition-row">
          <span class="t-from">{t.from}</span>
          <span class="t-arrow">→</span>
          <span class="t-to">{t.to}</span>
          <span class="t-time">{fmtTime(t.timestamp)}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .panel {
    background: var(--glass-bg);
    border: 1px solid var(--glass-border);
    border-radius: 12px;
    padding: 0.875rem 1rem;
    display: flex;
    flex-direction: column;
    gap: 0.625rem;
    font-size: 0.75rem;
    backdrop-filter: blur(var(--glass-blur));
  }

  .panel-header {
    display: flex;
    align-items: center;
    gap: 0.625rem;
  }

  .panel-title {
    font-size: 0.6875rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: var(--color-text-secondary);
    text-transform: uppercase;
    flex-shrink: 0;
  }

  .hint {
    font-size: 0.625rem;
    color: rgba(255,255,255,0.25);
    margin-right: auto;
  }

  .state-badge {
    font-size: 0.625rem;
    font-weight: 700;
    letter-spacing: 0.06em;
    padding: 0.125rem 0.375rem;
    border-radius: 4px;
    background: rgba(255,255,255,0.08);
    color: var(--color-text-secondary);
  }
  .state-badge.state-active { background: rgba(74,222,128,0.15); color: var(--color-status-green); }
  .state-badge.state-idle   { background: rgba(255,255,255,0.06); }
  .state-badge.state-done   { background: rgba(98,177,243,0.15); color: var(--color-highlight); }
  .state-badge.state-away   { background: rgba(251,191,36,0.15); color: var(--color-status-amber); }

  .idle-elapsed {
    font-size: 0.625rem;
    color: var(--color-text-secondary);
  }

  /* EMA / Score rows */
  .metrics { display: flex; flex-direction: column; gap: 0.375rem; }

  .metric-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .metric-label {
    width: 2.75rem;
    font-size: 0.6875rem;
    color: var(--color-text-secondary);
    flex-shrink: 0;
  }

  .bar-track {
    flex: 1;
    height: 6px;
    background: rgba(255,255,255,0.08);
    border-radius: 3px;
    position: relative;
    overflow: visible;
  }

  .bar-fill {
    height: 100%;
    border-radius: 3px;
    transition: width 0.4s ease;
  }
  .bar-fill.ema   { background: var(--color-accent); }
  .bar-fill.score { background: var(--color-text-secondary); }

  .threshold {
    position: absolute;
    top: -3px;
    width: 1.5px;
    height: 12px;
    border-radius: 1px;
  }
  .threshold.active-thresh { background: var(--color-status-green); }
  .threshold.idle-thresh   { background: var(--color-status-amber); }

  .metric-value {
    width: 2.75rem;
    text-align: right;
    font-family: 'SF Mono', 'Fira Code', monospace;
    color: var(--color-text-primary);
    flex-shrink: 0;
  }

  .threshold-legend {
    display: flex;
    gap: 0.5rem;
    padding-left: 3.25rem; /* align under bar-track, past the metric-label */
  }

  .thresh-pip-label {
    position: absolute;
    top: -14px;
    left: 50%;
    transform: translateX(-50%);
    font-size: 0.5rem;
    white-space: nowrap;
    color: inherit;
    pointer-events: none;
  }
  .threshold.idle-thresh .thresh-pip-label { color: var(--color-status-amber); }
  .threshold.active-thresh .thresh-pip-label { color: var(--color-status-green); }

  .threshold-labels {
    display: flex;
    gap: 0.375rem;
    flex-shrink: 0;
  }

  .thresh-tag {
    font-size: 0.5625rem;
    font-family: 'SF Mono', 'Fira Code', monospace;
    opacity: 0.7;
  }
  .thresh-tag.active { color: var(--color-status-green); }
  .thresh-tag.idle   { color: var(--color-status-amber); }

  /* Sparkline */
  .sparkline-wrap {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .sparkline {
    flex: 1;
    height: 32px;
    overflow: visible;
  }

  .sparkline-label {
    font-size: 0.5625rem;
    color: rgba(255,255,255,0.25);
    white-space: nowrap;
    flex-shrink: 0;
  }

  /* Features grid */
  .features {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 0.25rem 0.75rem;
  }

  .feature {
    display: flex;
    align-items: center;
    gap: 0.375rem;
  }

  .feature-label {
    width: 3.5rem;
    font-size: 0.625rem;
    color: var(--color-text-secondary);
    flex-shrink: 0;
  }

  .feature-track {
    flex: 1;
    height: 4px;
    background: rgba(255,255,255,0.08);
    border-radius: 2px;
    overflow: hidden;
  }

  .feature-fill {
    height: 100%;
    background: var(--color-accent-dark);
    border-radius: 2px;
    transition: width 0.4s ease;
  }

  .feature-value {
    width: 2.25rem;
    text-align: right;
    font-family: 'SF Mono', 'Fira Code', monospace;
    font-size: 0.5625rem;
    color: rgba(255,255,255,0.4);
    flex-shrink: 0;
  }

  /* Transitions */
  .transitions {
    border-top: 1px solid rgba(255,255,255,0.06);
    padding-top: 0.5rem;
    display: flex;
    flex-direction: column;
    gap: 0.2rem;
  }

  .section-label {
    font-size: 0.6rem;
    font-weight: 700;
    letter-spacing: 0.08em;
    color: rgba(255,255,255,0.25);
    text-transform: uppercase;
    margin-bottom: 0.125rem;
  }

  .transition-row {
    display: flex;
    align-items: center;
    gap: 0.375rem;
    font-size: 0.625rem;
  }

  .t-from { color: var(--color-text-secondary); }
  .t-arrow { color: rgba(255,255,255,0.25); }
  .t-to   { color: var(--color-text-primary); font-weight: 600; }
  .t-time { margin-left: auto; font-family: monospace; color: rgba(255,255,255,0.3); font-size: 0.5625rem; }

  .no-data {
    color: rgba(255,255,255,0.3);
    font-size: 0.6875rem;
    margin: 0;
    text-align: center;
    padding: 0.5rem 0;
  }
</style>
