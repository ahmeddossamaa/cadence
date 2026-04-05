# Cadence — Backend Implementation Plan (V0)

## Context

Cadence is a desktop time tracking app built with Tauri 2.0 (Rust backend + Svelte frontend). This is the V0 scope — core algorithm running locally, no external integrations. The goal is to run it for a day and validate the scoring, state detection, and adaptation loop.

---

## Architecture Overview

```
Controllers (IPC)
    ↓
Services
    Orchestrator → State Machine (core)
                 → Block service
                 → Prompt service
                 → Notification service
                 → Session service
                 → Feedback service
    ↓
Core
    State Machine — pure: state + signal → new state
    Scoring — pure: features + weights → score → EMA
    ↓
Workers
    Sampler — every 2s, reads platform, fills memory buffer
    Evaluator — every 5s, drains buffer, scores, calls orchestrator
    ↓
Adapters
    Persistence (SQLite) — blocks, session, feedback, calibration, settings
    ↓
Platform (OS)
    macOS — CoreGraphics input counters, AppleScript process info, iostat disk IO
    Linux — evdev input events, X11/Wayland process info, /proc/diskstats
```

Two inputs to the orchestrator:
1. Evaluator output (algorithm-driven transitions)
2. Controller calls (user-driven actions: pause, done, prompt response)

---

## Algorithm

### 1. Observation
Collect a normalized feature vector every 2 seconds:
```
X(t) = [keys, clicks, moves, scroll, cpu, process, stability]
```
Each value normalized to [0, 1]. Platform module handles OS-specific capture.

### 2. Scoring
Every 5 seconds, drain the sample buffer and compute:
```
score(t) = clamp(W · X(t), 0, 1)
ema(t) = α × score(t) + (1 - α) × ema(t-1)
α = 1 - e^(-S/H)
```
S = evaluate interval (5s), H = half-life (150s), α ≈ 0.033.

### 3. Classification (State Machine)
```
ema > θ_active → ACTIVE
ema < θ_idle for T_timeout → IDLE
θ_idle < ema < θ_active → no change (hysteresis)
```
θ_idle = 0.08, θ_active = 0.18, T_timeout = 300s. All adaptive.

States: IDLE, ACTIVE, AWAY, DONE.

Additional inputs (not from EMA):
- Screen lock → AWAY
- Screen unlock → record timestamp, let evaluator detect activity
- User action → pause, done

### 4. Adaptation
On each prompt response, receive a labeled pair (X_j, y_j):
```
error = y - score
w_i(new) = w_i(old) + η × error × x_i
normalize: w_i = max(w_i, 0) / Σ max(w_j, 0)

η(n) = η₀ × |error| / (1 + λ × n)
η₀ = 0.1, λ = 0.1, n = feedback sample count

θ_idle = mean(idle_scores) + σ(idle_scores)
θ_active = mean(active_scores) - σ(active_scores)
constraint: θ_active ≥ θ_idle + 0.05
```

---

## Database Schema

```sql
CREATE TABLE session (
    id              INTEGER PRIMARY KEY CHECK (id = 1),
    state           TEXT NOT NULL DEFAULT 'IDLE',
    block_id        INTEGER,
    untagged_secs   INTEGER NOT NULL DEFAULT 0,
    checkpoint_at   INTEGER NOT NULL,
    started_at      INTEGER NOT NULL
);

CREATE TABLE blocks (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    state           TEXT NOT NULL,
    started_at      INTEGER NOT NULL,
    ended_at        INTEGER,
    keys            INTEGER NOT NULL DEFAULT 0,
    clicks          INTEGER NOT NULL DEFAULT 0,
    moves           INTEGER NOT NULL DEFAULT 0,
    scroll          INTEGER NOT NULL DEFAULT 0,
    cpu             REAL NOT NULL DEFAULT 0,
    ema             REAL NOT NULL DEFAULT 0,
    app_switches    INTEGER NOT NULL DEFAULT 0,
    dominant_app    TEXT,
    source          TEXT NOT NULL DEFAULT 'SYSTEM',
    parent_id       INTEGER REFERENCES blocks(id)
);

CREATE TABLE feedback (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp       INTEGER NOT NULL,
    label           INTEGER NOT NULL,
    keys            REAL NOT NULL,
    clicks          REAL NOT NULL,
    moves           REAL NOT NULL,
    scroll          REAL NOT NULL,
    cpu             REAL NOT NULL,
    process         REAL NOT NULL,
    stability       REAL NOT NULL,
    ema             REAL NOT NULL
);

CREATE TABLE calibration (
    id              INTEGER PRIMARY KEY CHECK (id = 1),
    w_keys          REAL NOT NULL DEFAULT 0.2,
    w_clicks        REAL NOT NULL DEFAULT 0.15,
    w_moves         REAL NOT NULL DEFAULT 0.2,
    w_scroll        REAL NOT NULL DEFAULT 0.15,
    w_cpu           REAL NOT NULL DEFAULT 0.1,
    w_process       REAL NOT NULL DEFAULT 0.1,
    w_stability     REAL NOT NULL DEFAULT 0.1,
    idle_threshold  REAL NOT NULL DEFAULT 0.08,
    active_threshold REAL NOT NULL DEFAULT 0.18,
    learning_rate   REAL NOT NULL DEFAULT 0.1,
    samples         INTEGER NOT NULL DEFAULT 0,
    updated_at      INTEGER NOT NULL
);

CREATE TABLE settings (
    key             TEXT PRIMARY KEY,
    value           TEXT NOT NULL
);

CREATE INDEX idx_blocks_started ON blocks(started_at);
CREATE INDEX idx_feedback_ts ON feedback(timestamp);
```

---

## File Structure

```
src-tauri/src/
├── main.rs
├── lib.rs
│
├── controllers/
│   ├── mod.rs
│   ├── tracker.rs              # start, stop, get_status
│   ├── settings.rs             # get, update
│   ├── prompts.rs              # respond
│   └── export.rs               # csv
│
├── services/
│   ├── mod.rs
│   ├── orchestrator.rs
│   ├── block.rs
│   ├── prompt.rs
│   ├── feedback.rs
│   ├── notification.rs
│   └── session.rs
│
├── core/
│   ├── mod.rs
│   ├── state_machine.rs
│   └── scoring.rs
│
├── workers/
│   ├── mod.rs
│   ├── sampler.rs
│   └── evaluator.rs
│
├── adapters/
│   ├── mod.rs
│   └── persistence/
│       ├── mod.rs
│       ├── blocks.rs
│       ├── session.rs
│       ├── feedback.rs
│       ├── calibration.rs
│       └── settings.rs
│
├── platform/
│   ├── mod.rs
│   ├── macos.rs
│   └── linux.rs
│
└── types/
    ├── mod.rs
    ├── state.rs
    ├── block.rs
    ├── sample.rs
    ├── prompt.rs
    ├── notification.rs
    ├── calibration.rs
    └── settings.rs
```

---

## Design Patterns

- **Strategy** — platform module. macOS and Linux behind a shared trait.
- **State** — state machine. Each state defines its own transition rules.
- **Mediator** — orchestrator. Coordinates services without them knowing about each other.
- **Pipes and filters** — overall data flow.

---

## IPC Contract

### Frontend → Backend (Tauri commands)

| Command | Params | Returns |
|---|---|---|
| `tracker_start` | none | `TimerState` |
| `tracker_stop` | none | `TimerState` |
| `tracker_get_status` | none | `TimerState` |
| `settings_get` | none | `Settings` |
| `settings_update` | `Settings` | `Settings` |
| `prompt_respond` | `{ id, value }` | none |
| `export_csv` | `{ path? }` | `string` |

### Backend → Frontend (Tauri events)

| Event | Payload |
|---|---|
| `state_changed` | `{ from, to, timestamp }` |
| `prompt` | `{ id, message, actions, timeout }` |
| `notification` | `{ id, type, message, timestamp }` |
| `timer_tick` | `{ elapsed, daily_total, daily_target, state }` |

---

## Data Flow

### Continuous loop:
```
Platform (2s) → Sample → Buffer (memory)
Buffer (5s) → Scoring → EMA → State Machine → Transition?
Transition → Orchestrator → Block service (close/open)
                          → Prompt service (fire prompt?)
                          → Notification service (emit event)
                          → Session service (update singleton)
```

### User action:
```
Frontend → IPC → Controller → Orchestrator → same services
```

### Prompt response:
```
Frontend → IPC → Controller → Orchestrator → Feedback service (record pair)
                                            → Adaptation (update W, θ, η)
                                            → Block service (tag gap)
                                            → Session service (clear untagged)
```

### Checkpoint:
```
Every 5 min while a block is open:
Evaluator → Orchestrator → Block service (update ended_at + metrics)
                         → Session service (update checkpoint_at)
```

### Prompt delivery:
```
All prompts are delivered as OS system notifications (Tauri notification plugin).
This applies whether the overlay is open or closed.

Prompt service decides to fire → sends OS notification with action buttons
User responds at OS level → response routed back to Rust
Orchestrator receives response → Feedback service (record + adapt)
                               → Block service (tag gap if applicable)
                               → Session service (clear untagged)
Frontend sidebar is a read-only log of past prompts and their responses.
It does NOT handle prompt interaction — only displays history.
```

Prompt types:
- "Starting work?" — fires when EMA crosses active threshold from IDLE
- "Still here?" — fires when EMA stays below idle threshold for T_timeout
- "Back — tag gap as break?" — fires on screen unlock after AWAY
- "Over estimate, re-estimate?" — fires when block duration exceeds estimate (future, needs Jira)
- "Daily target reached. Done?" — fires when daily total hits target

---

## Persistence

- **Block lifecycle**: INSERT on state → ACTIVE. UPDATE every 5min (checkpoint). UPDATE on state change (close).
- **Session singleton**: One row. Updated on every checkpoint and state change. Read on startup for crash recovery.
- **Calibration singleton**: One row. Updated after each prompt response.
- **Settings**: Key-value pairs.

## Database Setup

**Location:** Tauri's app data directory via `app.path().app_data_dir()`.
- macOS: `~/Library/Application Support/com.ahmeddossamaa.cadence/cadence.db`
- Linux: `~/.local/share/com.ahmeddossamaa.cadence/cadence.db`

**Migrations:** Use `refinery` + `refinery-rusqlite`. Embedded in binary, run on startup.

```toml
refinery = { version = "0.8", features = ["rusqlite"] }
```

```
src-tauri/migrations/
├── V1__initial_schema.sql
```

On startup:
1. Resolve app data directory
2. Create directory if not exists
3. Open SQLite with WAL mode
4. Run pending migrations
5. Verify session singleton exists, create if not

---

## Implementation Order

### Phase 1 — Foundation
1. Types
2. Platform (carry over from daemon)
3. Persistence (schema + CRUD)
4. Settings

### Phase 2 — Algorithm
5. Scoring (pure functions)
6. State machine (pure function)
7. Adaptation (pure functions)

### Phase 3 — Workers
8. Sampler (2s loop)
9. Evaluator (5s loop)

### Phase 4 — Services
10. Session service
11. Block service
12. Prompt service
13. Notification service
14. Feedback service
15. Orchestrator

### Phase 5 — Controllers
16. IPC commands
17. Screen lock listener

### Phase 6 — Startup
18. Tauri setup, spawn workers, register commands, init DB, recover session

### Phase 7 — Prompt Delivery
19. System notifications via Tauri notification plugin
20. Prompt response handling from OS notification back to Rust
21. Wire prompt service to fire OS notifications on all prompt types
22. Frontend notification sidebar displays prompt history only (read-only log)

---

## Tunable Parameters (settings table)

| Key | Default |
|---|---|
| `sample_interval_secs` | `2` |
| `evaluate_interval_secs` | `5` |
| `checkpoint_interval_secs` | `300` |
| `ema_halflife_secs` | `150` |
| `idle_timeout_secs` | `300` |
| `prompt_timeout_secs` | `120` |
| `prompt_cooldown_secs` | `300` |
| `prompt_debounce_secs` | `60` |
| `daily_target_secs` | `28800` |

---

## Naming Rules

- No comments unless genuinely non-obvious
- Self-explanatory names, rename instead of commenting
- Modules: snake_case
- Structs/Enums: PascalCase
- Functions: snake_case
- Constants: SCREAMING_SNAKE_CASE
- No abbreviations except: id, url, pid, ema

---

## What NOT to Build (V0)

- No Jira integration
- No Calendar integration
- No ticket management
- No window title capture
- No keystroke content capture
- No screenshot capture
- No mic/audio detection
- No memory usage tracking
- No settings UI
- No system tray
- No multi-user support
- No central server