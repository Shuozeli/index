use crate::display::table_renderer::RepoStatusRow;

pub fn render_report_markdown(
    rows: &[RepoStatusRow],
    period: &str,
    category_breakdown: &[(String, u32)],
) -> String {
    let mut out = String::new();

    out.push_str(&format!("# pidx Report ({period})\n\n"));
    out.push_str(&format!(
        "Generated: {}\n\n",
        chrono::Utc::now().format("%Y-%m-%d %H:%M UTC")
    ));

    // Summary table
    out.push_str("## Repository Status\n\n");
    out.push_str("| Repo | Category | Lang | Issues | Last Push | Velocity | Health | LLM Status |\n");
    out.push_str("|------|----------|------|--------|-----------|----------|--------|------------|\n");

    for row in rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {}/30d | {:.0} {} | {} |\n",
            row.name,
            row.category,
            row.language,
            row.open_issues,
            row.last_push,
            row.commits_30d,
            row.health_score,
            row.health_label,
            row.llm_status.as_deref().unwrap_or("-"),
        ));
    }

    // Category breakdown
    out.push_str("\n## Commit Breakdown by Category\n\n");
    out.push_str("| Category | Commits |\n");
    out.push_str("|----------|---------|\n");
    for (cat, count) in category_breakdown {
        out.push_str(&format!("| {} | {} |\n", cat, count));
    }

    // LLM summaries
    let has_llm = rows.iter().any(|r| r.llm_status.is_some());
    if has_llm {
        out.push_str("\n## LLM Insights\n\n");
        for row in rows {
            if let Some(status) = &row.llm_status {
                out.push_str(&format!("### {}\n\n{}\n\n", row.name, status));
            }
        }
    }

    out
}
