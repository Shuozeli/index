use anyhow::Context;
use rusqlite::{Connection, params};

pub fn log_sync_event(
    conn: &Connection,
    repo_name: &str,
    event_type: &str,
    detail: Option<&str>,
) -> anyhow::Result<()> {
    conn.execute(
        "INSERT INTO sync_events (repo_name, event_type, detail) VALUES (?1, ?2, ?3)",
        params![repo_name, event_type, detail],
    )
    .context("Failed to log sync event")?;
    Ok(())
}
