use std::collections::HashMap;

use chrono::{DateTime, Duration, Utc};

use crate::config::Config;
use crate::db::Database;
use crate::db::commit_store;
use crate::db::llm_summary_store;
use crate::db::repo_store;
use crate::display::markdown_renderer::render_report_markdown;
use crate::display::table_renderer::{RepoStatusRow, render_status_table};
use crate::health::compute_health;

pub fn run(config: &Config, format: &str, period: &str) -> anyhow::Result<()> {
    let db = Database::open(&config.db_path())?;

    let repos = db.tx(|conn| repo_store::get_all_repos(conn))?;

    if repos.is_empty() {
        println!("No repos synced yet. Run `pidx sync` first.");
        return Ok(());
    }

    let thirty_days_ago = (Utc::now() - Duration::days(30))
        .format("%Y-%m-%dT%H:%M:%SZ")
        .to_string();

    let period_since = parse_period(period)?;
    let period_since_str = period_since.format("%Y-%m-%dT%H:%M:%SZ").to_string();

    let mut rows = Vec::new();
    let mut category_counts: HashMap<String, u32> = HashMap::new();

    for repo in &repos {
        let commits_30d = db.tx(|conn| {
            commit_store::count_commits_since(conn, repo.id, &thirty_days_ago)
        })?;

        let commits_period = db.tx(|conn| {
            commit_store::get_commits_since(conn, repo.id, &period_since_str)
        })?;

        for c in &commits_period {
            *category_counts.entry(c.category.clone()).or_insert(0) += 1;
        }

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

    let mut category_breakdown: Vec<(String, u32)> = category_counts.into_iter().collect();
    category_breakdown.sort_by(|a, b| b.1.cmp(&a.1));

    match format {
        "md" | "markdown" => {
            let md = render_report_markdown(&rows, period, &category_breakdown);
            println!("{md}");
        }
        _ => {
            println!("Report for period: {period}\n");
            render_status_table(&rows);
            println!("\nCommit breakdown:");
            for (cat, count) in &category_breakdown {
                println!("  {cat}: {count}");
            }
        }
    }

    Ok(())
}

fn parse_period(s: &str) -> anyhow::Result<DateTime<Utc>> {
    let now = Utc::now();
    let s = s.trim();

    if let Some(days_str) = s.strip_suffix('d') {
        let days: i64 = days_str.parse()?;
        Ok(now - Duration::days(days))
    } else if let Some(weeks_str) = s.strip_suffix('w') {
        let weeks: i64 = weeks_str.parse()?;
        Ok(now - Duration::weeks(weeks))
    } else {
        anyhow::bail!("Invalid period format: {s}. Use e.g. 7d, 2w");
    }
}
