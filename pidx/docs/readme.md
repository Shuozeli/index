# pidx

Director-level project index CLI. Syncs GitHub data into local SQLite, generates structured docs for LLM analysis, and presents health/velocity dashboards.

## Setup

1. Create config at `~/.pidx/pidx.toml` (see `pidx.toml.example`)
2. Set `GITHUB_TOKEN` environment variable
3. `cargo build --release`

## Usage

```bash
pidx sync                          # Pull data from GitHub
pidx status                        # Table overview
pidx activity --since 7d           # Recent commits
pidx report --format md --period 7d # Weekly digest
pidx docs export                   # Generate markdown for LLM
pidx docs ingest                   # Read LLM analysis back
pidx config show                   # Display config
```

## LLM Integration

1. Run `pidx docs export` to generate per-repo markdown at `~/.pidx/docs/{repo}/`
2. Feed the docs to an LLM (e.g., Gemini)
3. LLM writes `llm_summary.md` back to each repo's doc dir
4. Run `pidx docs ingest` to store in SQLite
5. `pidx status` and `pidx report` now include LLM insights
