import { writable } from 'svelte/store';
import type { Diagnostics, TransitionEntry } from '../types/diagnostics';

const MAX_HISTORY = 60;
const MAX_TRANSITIONS = 8;

interface DiagnosticsState {
  current: Diagnostics | null;
  emaHistory: number[];
  transitions: TransitionEntry[];
}

function createDiagnosticsStore() {
  const { subscribe, update } = writable<DiagnosticsState>({
    current: null,
    emaHistory: [],
    transitions: [],
  });

  return {
    subscribe,
    onDiagnostics(d: Diagnostics) {
      update(s => ({
        ...s,
        current: d,
        emaHistory: [...s.emaHistory, d.ema].slice(-MAX_HISTORY),
      }));
    },
    onTransition(from: string, to: string, timestamp: number) {
      update(s => ({
        ...s,
        transitions: [{ from, to, timestamp }, ...s.transitions].slice(0, MAX_TRANSITIONS),
      }));
    },
  };
}

export const diagnosticsStore = createDiagnosticsStore();
