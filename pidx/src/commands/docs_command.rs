use std::fs;
use std::path::Path;

use chrono::{DateTime, Duration, Utc};

use crate::config::Config;
use crate::db::Database;
use crate::db::commit_store;
use crate::db::issue_store;
use crate::db::llm_summary_store;
use crate::db::release_store;
use crate::db::repo_store;
use crate::health::compute_health;

pub fn export(config: &Config, repo_filter: Option<&str>) -> anyhow::Result<()> {
    let db = Database::open(&config.db_path())?;
    let repos = db.tx(|conn| repo_store::get_all_repos(conn))?;

    for repo in &repos {
        if let Some(filter) = repo_filter {
            if repo.name != filter {
                continue;
            }
        }

        let docs_dir = Config::repo_docs_dir(&repo.name);
        fs::create_dir_all(&docs_dir)?;

        let now = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();

        // overview.md
        write_doc(
            &docs_dir.join("overview.md"),
            &format!(
                "---\ngenerated_at: {now}\nrepo: {}\nperiod: all\n---\n\n# {}\n\n- **Owner:** {}\n- **Language:** {}\n- **Description:** {}\n- **Open Issues:** {}\n- **Last Push:** {}\n- **Category:** {}\n",
                repo.name,
                repo.name,
                repo.owner,
                repo.language.as_deref().unwrap_or("unknown"),
                repo.description.as_deref().unwrap_or("N/A"),
                repo.open_issues,
                repo.pushed_at.as_deref().unwrap_or("never"),
                repo.category.as_deref().unwrap_or("uncategorized"),
            ),
        )?;

        // changelog.md
        let commits = db.tx(|conn| commit_store::get_all_commits_for_repo(conn, repo.id))?;
        let mut changelog = format!(
            "---\ngenerated_at: {now}\nrepo: {}\nperiod: all\n---\n\n# Changelog\n\n",
            repo.name
        );
        let mut current_date = String::new();
        for c in &commits {
            let date = c
                .committed_at
                .parse::<DateTime<Utc>>()
                .map(|dt| dt.format("%Y-%m-%d").to_string())
                .unwrap_or_else(|_| "unknown".to_string());
            if date != current_date {
                changelog.push_str(&format!("\n## {date}\n\n"));
                current_date = date;
            }
            changelog.push_str(&format!(
                "- **[{}]** `{}` {}\n",
                c.category,
                &c.sha[..7.min(c.sha.len())],
                c.message
            ));
        }
        write_doc(&docs_dir.join("changelog.md"), &changelog)?;

        // issues.md
        let all_issues = db.tx(|conn| issue_store::get_all_issues_for_repo(conn, repo.id))?;
        let mut issues_md = format!(
            "---\ngenerated_at: {now}\nrepo: {}\nperiod: all\n---\n\n# Issues\n\n",
            repo.name
        );
        let open: Vec<_> = all_issues.iter().filter(|i| i.state == "open").collect();
        let closed: Vec<_> = all_issues.iter().filter(|i| i.state == "closed").collect();
        issues_md.push_str(&format!(
            "**Open:** {} | **Closed:** {}\n\n",
            open.len(),
            closed.len()
        ));
        if !open.is_empty() {
            issues_md.push_str("## Open Issues\n\n");
            for i in &open {
                issues_md.push_str(&format!("- #{} {}\n", i.number, i.title));
            }
        }
        if !closed.is_empty() {
            issues_md.push_str("\n## Recently Closed\n\n");
            for i in closed.iter().take(10) {
                issues_md.push_str(&format!("- #{} {}\n", i.number, i.title));
            }
        }
        write_doc(&docs_dir.join("issues.md"), &issues_md)?;

        // releases.md
        let releases = db.tx(|conn| release_store::get_releases_for_repo(conn, repo.id))?;
        let mut releases_md = format!(
            "---\ngenerated_at: {now}\nrepo: {}\nperiod: all\n---\n\n# Releases\n\n",
            repo.name
        );
        if releases.is_empty() {
            releases_md.push_str("No releases found.\n");
        } else {
            for rel in &releases {
                releases_md.push_str(&format!(
                    "## {} {}\n\nPublished: {}\n\n{}\n\n",
                    rel.tag_name,
                    rel.name.as_deref().unwrap_or(""),
                    rel.published_at.as_deref().unwrap_or("unknown"),
                    rel.body.as_deref().unwrap_or(""),
                ));
            }
        }
        write_doc(&docs_dir.join("releases.md"), &releases_md)?;

        // health.md
        let thirty_days_ago = (Utc::now() - Duration::days(30))
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();
        let commits_30d = db.tx(|conn| {
            commit_store::count_commits_since(conn, repo.id, &thirty_days_ago)
        })?;
        let last_pushed: Option<DateTime<Utc>> = repo
            .pushed_at
            .as_deref()
            .and_then(|s| s.parse().ok());
        let health = compute_health(last_pushed, commits_30d, repo.open_issues as u32);

        let health_md = format!(
            "---\ngenerated_at: {now}\nrepo: {}\nperiod: 30d\n---\n\n# Health Score\n\n- **Total:** {:.0}/100 ({})\n- **Recency:** {:.0}/100 (weight: 40%)\n- **Velocity:** {:.0}/100 (weight: 40%, {} commits/30d)\n- **Issues:** {:.0}/100 (weight: 20%, {} open)\n",
            repo.name,
            health.total,
            health.label,
            health.recency,
            health.velocity,
            commits_30d,
            health.issues,
            repo.open_issues,
        );
        write_doc(&docs_dir.join("health.md"), &health_md)?;

        println!("  Exported docs for {} -> {}", repo.name, docs_dir.display());
    }

    println!("Docs export complete.");
    Ok(())
}

pub fn ingest(config: &Config, repo_filter: Option<&str>) -> anyhow::Result<()> {
    let db = Database::open(&config.db_path())?;
    let repos = db.tx(|conn| repo_store::get_all_repos(conn))?;

    let mut ingested = 0;
    for repo in &repos {
        if let Some(filter) = repo_filter {
            if repo.name != filter {
                continue;
            }
        }

        let summary_path = Config::repo_docs_dir(&repo.name).join("llm_summary.md");
        if !summary_path.exists() {
            continue;
        }

        let content = fs::read_to_string(&summary_path)?;
        let (frontmatter, body) = parse_frontmatter(&content);

        let analyzed_at = frontmatter
            .get("analyzed_at")
            .cloned()
            .unwrap_or_else(|| Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        let model = frontmatter.get("model").cloned();

        // Parse sections from body
        let status_summary = extract_section(&body, "## Project Status");
        let risks = extract_section(&body, "## Key Risks");
        let recommendations = extract_section(&body, "## Recommendations");

        db.tx(|conn| {
            llm_summary_store::insert_llm_summary(
                conn,
                repo.id,
                &analyzed_at,
                model.as_deref(),
                status_summary.as_deref(),
                risks.as_deref(),
                recommendations.as_deref(),
                &content,
            )?;
            Ok(())
        })?;

        println!("  Ingested LLM summary for {}", repo.name);
        ingested += 1;
    }

    if ingested == 0 {
        println!("No llm_summary.md files found. Place them at ~/.pidx/docs/<repo>/llm_summary.md");
    } else {
        println!("Ingested {ingested} LLM summaries.");
    }

    Ok(())
}

fn write_doc(path: &Path, content: &str) -> anyhow::Result<()> {
    fs::write(path, content)?;
    Ok(())
}

fn parse_frontmatter(content: &str) -> (std::collections::HashMap<String, String>, String) {
    let mut map = std::collections::HashMap::new();
    let content = content.trim();

    if !content.starts_with("---") {
        return (map, content.to_string());
    }

    let after_first = &content[3..];
    if let Some(end_idx) = after_first.find("---") {
        let fm_block = &after_first[..end_idx];
        let body = after_first[end_idx + 3..].trim().to_string();

        for line in fm_block.lines() {
            let line = line.trim();
            if let Some((key, value)) = line.split_once(':') {
                map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        (map, body)
    } else {
        (map, content.to_string())
    }
}

fn extract_section(body: &str, heading: &str) -> Option<String> {
    let start = body.find(heading)?;
    let after_heading = &body[start + heading.len()..];

    // Find next ## heading or end of string
    let end = after_heading
        .find("\n## ")
        .unwrap_or(after_heading.len());

    let section = after_heading[..end].trim().to_string();
    if section.is_empty() {
        None
    } else {
        Some(section)
    }
}
