CREATE TABLE IF NOT EXISTS workforce_snapshot (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    source TEXT NOT NULL,
    employees JSONB NOT NULL,
    snapshotted_at TIMESTAMP NOT NULL
);
