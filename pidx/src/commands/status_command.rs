use chrono::{DateTime, Utc};

use crate::config::Config;
use crate::db::Database;
use crate::db::commit_store;
use crate::db::llm_summary_store;
use crate::db::repo_store;
use crate::display::table_renderer::{RepoStatusRow, render_status_table};
use crate::health::compute_health;

pub fn run(config: &Config) -> anyhow::Result<()> {
    let db = Database::open(&config.db_path())?;

    let repos = db.tx(|conn| repo_store::get_all_repos(conn))?;

    if repos.is_empty() {
        println!("No repos synced yet. Run `pidx sync` first.");
        return Ok(());
    }

    let thirty_days_ago = (Utc::now() - chrono::Duration::days(30))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();

    let mut rows = Vec::new();
    for repo in &repos {
        let commits_30d = db.tx(|conn| {
            commit_store::count_commits_since(conn, repo.id, &thirty_days_ago)
        })?;

        let last_pushed: Option<DateTime<Utc>> = repo
            .pushed_at
            .as_deref()
            .and_then(|s| s.parse().ok());

        let health = compute_health(last_pushed, commits_30d, repo.open_issues as u32);

        let llm_status = db.tx(|conn| {
            let summary = llm_summary_store::get_latest_summary(conn, repo.id)?;
            Ok(summary.and_then(|s| s.status_summary))
        })?;

        let last_push_display = repo
            .pushed_at
            .as_deref()
            .and_then(|s| s.parse::<DateTime<Utc>>().ok())
            .map(|dt| {
                let days = (Utc::now() - dt).num_days();
                if days == 0 {
                    "today".to_string()
                } else if days == 1 {
                    "1d ago".to_string()
                } else {
                    format!("{days}d ago")
                }
            })
            .unwrap_or_else(|| "never".to_string());

        rows.push(RepoStatusRow {
            name: repo.name.clone(),
            category: repo.category.clone().unwrap_or_default(),
            language: repo.language.clone().unwrap_or_else(|| "-".to_string()),
            open_issues: repo.open_issues,
            last_push: last_push_display,
            commits_30d,
            health_score: health.total,
            health_label: health.label,
            llm_status,
        });
    }

    render_status_table(&rows);
    Ok(())
}
