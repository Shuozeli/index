use anyhow::Context;
use rusqlite::{Connection, params};

#[derive(Debug, Clone)]
pub struct LlmSummaryRow {
    pub id: i64,
    pub repo_id: i64,
    pub analyzed_at: String,
    pub model: Option<String>,
    pub status_summary: Option<String>,
    pub risks: Option<String>,
    pub recommendations: Option<String>,
    pub raw_content: String,
    pub ingested_at: String,
}

pub fn insert_llm_summary(
    conn: &Connection,
    repo_id: i64,
    analyzed_at: &str,
    model: Option<&str>,
    status_summary: Option<&str>,
    risks: Option<&str>,
    recommendations: Option<&str>,
    raw_content: &str,
) -> anyhow::Result<()> {
    conn.execute(
        r#"INSERT INTO llm_summaries (repo_id, analyzed_at, model, status_summary, risks, recommendations, raw_content, ingested_at)
           VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, datetime('now'))"#,
        params![repo_id, analyzed_at, model, status_summary, risks, recommendations, raw_content],
    )
    .context("Failed to insert LLM summary")?;
    Ok(())
}

pub fn get_latest_summary(
    conn: &Connection,
    repo_id: i64,
) -> anyhow::Result<Option<LlmSummaryRow>> {
    let mut stmt = conn
        .prepare(
            "SELECT id, repo_id, analyzed_at, model, status_summary, risks, recommendations, raw_content, ingested_at
             FROM llm_summaries
             WHERE repo_id = ?1
             ORDER BY analyzed_at DESC
             LIMIT 1",
        )
        .context("Failed to prepare LLM summary query")?;

    let mut rows = stmt
        .query_map(params![repo_id], |row| {
            Ok(LlmSummaryRow {
                id: row.get(0)?,
                repo_id: row.get(1)?,
                analyzed_at: row.get(2)?,
                model: row.get(3)?,
                status_summary: row.get(4)?,
                risks: row.get(5)?,
                recommendations: row.get(6)?,
                raw_content: row.get(7)?,
                ingested_at: row.get(8)?,
            })
        })
        .context("Failed to query LLM summaries")?;

    match rows.next() {
        Some(row) => Ok(Some(row.context("Failed to read LLM summary row")?)),
        None => Ok(None),
    }
}
