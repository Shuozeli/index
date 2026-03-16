use tracing::info;

use crate::classify::CommitCategory;
use crate::config::Config;
use crate::db::Database;
use crate::db::commit_store;
use crate::db::issue_store;
use crate::db::release_store;
use crate::db::repo_store;
use crate::db::sync_log_store;
use crate::github::GithubClient;

pub async fn run(config: &Config, repo_filter: Option<&str>) -> anyhow::Result<()> {
    let token = config.github_token()?;
    let client = GithubClient::new(&token, &config.owner)?;
    let db = Database::open(&config.db_path())?;

    let repos: Vec<_> = config
        .repos
        .iter()
        .filter(|r| repo_filter.is_none() || repo_filter == Some(r.name.as_str()))
        .collect();

    if repos.is_empty() {
        anyhow::bail!("No matching repos found in config");
    }

    for repo_entry in &repos {
        let name = &repo_entry.name;
        info!("Syncing {name}...");

        // Fetch repo metadata
        let gh_repo = client.fetch_repo(name).await?;
        let repo_id = db.tx(|conn| {
            let id = repo_store::upsert_repo(
                conn,
                &config.owner,
                name,
                gh_repo.language.as_deref(),
                gh_repo.description.as_deref(),
                gh_repo.open_issues_count,
                gh_repo.pushed_at.as_deref(),
                Some(&repo_entry.category),
            )?;
            sync_log_store::log_sync_event(conn, name, "repo_synced", None)?;
            Ok(id)
        })?;

        // Fetch commits
        let commits = client
            .fetch_commits(name, config.sync.commits_per_sync)
            .await?;
        let commit_count = commits.len();
        db.tx(|conn| {
            for c in &commits {
                let message = c.commit.message.lines().next().unwrap_or("");
                let author = c
                    .author
                    .as_ref()
                    .map(|a| a.login.as_str())
                    .or_else(|| c.commit.author.as_ref().and_then(|a| a.name.as_deref()));
                let date = c
                    .commit
                    .author
                    .as_ref()
                    .and_then(|a| a.date.as_deref())
                    .unwrap_or("");
                let category = CommitCategory::from_message(message);

                commit_store::upsert_commit(
                    conn,
                    repo_id,
                    &c.sha,
                    message,
                    author,
                    date,
                    category.as_str(),
                )?;
            }
            sync_log_store::log_sync_event(
                conn,
                name,
                "commits_synced",
                Some(&format!("{commit_count} commits")),
            )?;
            Ok(())
        })?;

        // Fetch issues (all states)
        let issues = client.fetch_issues(name, "all").await?;
        let issue_count = issues.len();
        db.tx(|conn| {
            for issue in &issues {
                let labels: Vec<&str> = issue.labels.iter().map(|l| l.name.as_str()).collect();
                let labels_json = serde_json::to_string(&labels)?;
                issue_store::upsert_issue(
                    conn,
                    repo_id,
                    issue.number,
                    &issue.title,
                    &issue.state,
                    &labels_json,
                    &issue.created_at,
                    issue.updated_at.as_deref(),
                    issue.closed_at.as_deref(),
                )?;
            }
            sync_log_store::log_sync_event(
                conn,
                name,
                "issues_synced",
                Some(&format!("{issue_count} issues")),
            )?;
            Ok(())
        })?;

        // Fetch releases
        let releases = client.fetch_releases(name).await?;
        let release_count = releases.len();
        db.tx(|conn| {
            for rel in &releases {
                release_store::upsert_release(
                    conn,
                    repo_id,
                    &rel.tag_name,
                    rel.name.as_deref(),
                    rel.body.as_deref(),
                    rel.published_at.as_deref(),
                )?;
            }
            sync_log_store::log_sync_event(
                conn,
                name,
                "releases_synced",
                Some(&format!("{release_count} releases")),
            )?;
            Ok(())
        })?;

        println!(
            "  {name}: {commit_count} commits, {issue_count} issues, {release_count} releases"
        );
    }

    println!("Sync complete.");
    Ok(())
}
