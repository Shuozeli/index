# pidx Architecture

## Overview

pidx is a director-level project index CLI that syncs GitHub data into local SQLite, generates structured docs for LLM consumption, and ingests LLM-produced analysis back. It provides a hybrid human+LLM dashboard for tracking progress, velocity, and health across multiple repositories.

## Data Flow

```
pidx sync  -->  GitHub API  -->  SQLite (raw data)
pidx docs export  -->  per-project markdown  -->  ~/.pidx/docs/{repo}/
LLM processes docs  -->  writes analysis back  -->  ~/.pidx/docs/{repo}/llm_summary.md
pidx docs ingest  -->  reads LLM output  -->  SQLite (llm_summaries table)
pidx status / report  -->  combines raw data + LLM summaries
```

## Module Structure

```
src/
  main.rs              -- clap CLI, command dispatch
  config.rs            -- TOML config loading (fail-fast)
  classify.rs          -- CommitCategory enum from message prefixes
  health.rs            -- Health score computation (recency + velocity + issues)
  db/                  -- SQLite via rusqlite, all access in transactions
    mod.rs             -- Database struct, migrations, tx() wrapper
    schema.rs          -- CREATE TABLE DDL
    *_store.rs         -- Per-entity CRUD (repo, commit, issue, release, llm_summary, sync_log)
  github/              -- GitHub API client via reqwest
    mod.rs             -- GithubClient (shared HTTP client with auth)
    types.rs           -- API response deserialization structs
    *_fetcher.rs       -- Per-entity fetch methods
  commands/            -- One module per CLI command
    sync_command.rs    -- Pulls data from GitHub, stores in SQLite
    status_command.rs  -- Table overview with health scores
    activity_command.rs-- Recent commits grouped by day
    report_command.rs  -- Digest with category breakdown
    docs_command.rs    -- Export markdown / ingest LLM summaries
  display/             -- Output rendering
    table_renderer.rs  -- comfy-table terminal tables
    markdown_renderer.rs -- Markdown report generation
```

## Design Decisions

- **Allowlist model**: Config is manually edited TOML. No auto-discovery.
- **All DB access in transactions**: Including reads, via `db.tx()`.
- **Fail-fast**: Missing config or env vars cause immediate errors.
- **Commit classification**: Prefix-based (feat:, fix:, etc.), defaults to Chore.
- **Health score**: Weighted composite of recency (40%), velocity (40%), issues (20%).
