use chrono::{DateTime, Duration, Utc};

use crate::config::Config;
use crate::db::Database;
use crate::db::commit_store;
use crate::db::repo_store;
use crate::display::table_renderer::{ActivityRow, render_activity_table};

pub fn run(config: &Config, repo_filter: Option<&str>, since: &str) -> anyhow::Result<()> {
    let db = Database::open(&config.db_path())?;

    let since_dt = parse_duration(since)?;
    let since_str = since_dt.format("%Y-%m-%dT%H:%M:%SZ").to_string();

    let repos = db.tx(|conn| repo_store::get_all_repos(conn))?;

    let mut activity_rows = Vec::new();

    for repo in &repos {
        if let Some(filter) = repo_filter {
            if repo.name != filter {
                continue;
            }
        }

        let commits = db.tx(|conn| {
            commit_store::get_commits_since(conn, repo.id, &since_str)
        })?;

        for commit in &commits {
            let date = commit
                .committed_at
                .parse::<DateTime<Utc>>()
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|_| commit.committed_at.clone());

            activity_rows.push(ActivityRow {
                date,
                repo: repo.name.clone(),
                sha_short: commit.sha.chars().take(7).collect(),
                category: commit.category.clone(),
                message: commit.message.chars().take(60).collect(),
            });
        }
    }

    // Sort by date descending
    activity_rows.sort_by(|a, b| b.date.cmp(&a.date));

    if activity_rows.is_empty() {
        println!("No activity found since {since}.");
    } else {
        println!("Activity since {since} ({} commits):\n", activity_rows.len());
        render_activity_table(&activity_rows);
    }

    Ok(())
}

fn parse_duration(s: &str) -> anyhow::Result<DateTime<Utc>> {
    let now = Utc::now();
    let s = s.trim();

    if let Some(days_str) = s.strip_suffix('d') {
        let days: i64 = days_str.parse()?;
        Ok(now - Duration::days(days))
    } else if let Some(weeks_str) = s.strip_suffix('w') {
        let weeks: i64 = weeks_str.parse()?;
        Ok(now - Duration::weeks(weeks))
    } else {
        anyhow::bail!("Invalid duration format: {s}. Use e.g. 7d, 2w");
    }
}
