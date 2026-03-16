# pidx Codelabs

## Codelab 1: Adding a New Repo

1. Edit `~/.pidx/pidx.toml`
2. Add a new `[[repos]]` entry with `name` and `category`
3. Run `pidx sync --repo <name>`
4. Verify with `pidx status`

## Codelab 2: Full LLM Analysis Loop

1. `pidx sync` to pull latest data
2. `pidx docs export` to generate markdown
3. Feed `~/.pidx/docs/*/` to your LLM
4. LLM writes `llm_summary.md` per repo with frontmatter
5. `pidx docs ingest` to store analysis
6. `pidx report --format md` to see blended report

## Codelab 3: Adding a New Command

1. Create `src/commands/my_command.rs` with a `pub fn run(...)` function
2. Add `pub mod my_command;` to `src/commands/mod.rs`
3. Add variant to `Commands` enum in `src/main.rs`
4. Wire dispatch in the `match` block
5. Use `db.tx(|conn| { ... })` for all DB access

## Codelab 4: Adding a New DB Table

1. Add CREATE TABLE statement to `src/db/schema.rs`
2. Create `src/db/my_store.rs` with CRUD functions
3. Add `pub mod my_store;` to `src/db/mod.rs`
4. All functions take `&Connection` (from the tx wrapper)
