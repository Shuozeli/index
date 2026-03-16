pub const CREATE_TABLES: &str = r#"
CREATE TABLE IF NOT EXISTS repos (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    owner       TEXT NOT NULL,
    name        TEXT NOT NULL,
    language    TEXT,
    description TEXT,
    open_issues INTEGER NOT NULL DEFAULT 0,
    pushed_at   TEXT,
    synced_at   TEXT,
    category    TEXT,
    UNIQUE(owner, name)
);

CREATE TABLE IF NOT EXISTS commits (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    repo_id      INTEGER NOT NULL REFERENCES repos(id),
    sha          TEXT NOT NULL,
    message      TEXT NOT NULL,
    author       TEXT,
    committed_at TEXT NOT NULL,
    category     TEXT NOT NULL,
    UNIQUE(repo_id, sha)
);

CREATE TABLE IF NOT EXISTS issues (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    repo_id    INTEGER NOT NULL REFERENCES repos(id),
    number     INTEGER NOT NULL,
    title      TEXT NOT NULL,
    state      TEXT NOT NULL,
    labels     TEXT NOT NULL DEFAULT '[]',
    created_at TEXT NOT NULL,
    updated_at TEXT,
    closed_at  TEXT,
    UNIQUE(repo_id, number)
);

CREATE TABLE IF NOT EXISTS releases (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    repo_id      INTEGER NOT NULL REFERENCES repos(id),
    tag_name     TEXT NOT NULL,
    name         TEXT,
    body         TEXT,
    published_at TEXT,
    UNIQUE(repo_id, tag_name)
);

CREATE TABLE IF NOT EXISTS llm_summaries (
    id              INTEGER PRIMARY KEY AUTOINCREMENT,
    repo_id         INTEGER NOT NULL REFERENCES repos(id),
    analyzed_at     TEXT NOT NULL,
    model           TEXT,
    status_summary  TEXT,
    risks           TEXT,
    recommendations TEXT,
    raw_content     TEXT NOT NULL,
    ingested_at     TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sync_events (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    repo_name  TEXT NOT NULL,
    event_type TEXT NOT NULL,
    detail     TEXT,
    created_at TEXT NOT NULL DEFAULT (datetime('now'))
);
"#;
