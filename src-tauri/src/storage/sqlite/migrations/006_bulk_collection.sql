-- SQLite schema migration: bulk collection tables (version 6)

CREATE TABLE IF NOT EXISTS bulk_collection_runs (
    run_id TEXT PRIMARY KEY,
    started_at_utc TEXT NOT NULL,
    completed_at_utc TEXT,
    status TEXT NOT NULL CHECK(status IN ('running', 'completed', 'cancelled')),
    total_targets INTEGER NOT NULL,
    completed_count INTEGER NOT NULL DEFAULT 0,
    failed_count INTEGER NOT NULL DEFAULT 0,
    success_count INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS bulk_collection_results (
    run_id TEXT NOT NULL,
    project_id INTEGER NOT NULL,
    branch_name TEXT NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('pending', 'success', 'failed')),
    new_commits_count INTEGER,
    error_message TEXT,
    processed_at_utc TEXT,
    PRIMARY KEY (run_id, project_id, branch_name),
    FOREIGN KEY (run_id) REFERENCES bulk_collection_runs(run_id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_bulk_results_run_status
    ON bulk_collection_results(run_id, status);

CREATE INDEX IF NOT EXISTS idx_bulk_results_target_status
    ON bulk_collection_results(project_id, branch_name, status);

CREATE INDEX IF NOT EXISTS idx_bulk_runs_status_started
    ON bulk_collection_runs(status, started_at_utc);
