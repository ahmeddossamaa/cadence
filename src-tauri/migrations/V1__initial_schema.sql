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
