use comfy_table::{Cell, Color, ContentArrangement, Table};

use crate::health::HealthLabel;

pub struct RepoStatusRow {
    pub name: String,
    pub category: String,
    pub language: String,
    pub open_issues: i32,
    pub last_push: String,
    pub commits_30d: u32,
    pub health_score: f64,
    pub health_label: HealthLabel,
    pub llm_status: Option<String>,
}

pub fn render_status_table(rows: &[RepoStatusRow]) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec![
        "Repo",
        "Category",
        "Lang",
        "Issues",
        "Last Push",
        "Velocity",
        "Health",
        "LLM Status",
    ]);

    for row in rows {
        let health_color = match row.health_label {
            HealthLabel::Active => Color::Green,
            HealthLabel::Healthy => Color::Cyan,
            HealthLabel::Moderate => Color::Yellow,
            HealthLabel::Stale => Color::Red,
            HealthLabel::Dormant => Color::DarkRed,
        };

        table.add_row(vec![
            Cell::new(&row.name),
            Cell::new(&row.category),
            Cell::new(&row.language),
            Cell::new(row.open_issues),
            Cell::new(&row.last_push),
            Cell::new(format!("{}/30d", row.commits_30d)),
            Cell::new(format!("{:.0} {}", row.health_score, row.health_label))
                .fg(health_color),
            Cell::new(row.llm_status.as_deref().unwrap_or("-")),
        ]);
    }

    println!("{table}");
}

pub struct ActivityRow {
    pub date: String,
    pub repo: String,
    pub sha_short: String,
    pub category: String,
    pub message: String,
}

pub fn render_activity_table(rows: &[ActivityRow]) {
    let mut table = Table::new();
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(vec!["Date", "Repo", "SHA", "Category", "Message"]);

    for row in rows {
        table.add_row(vec![
            Cell::new(&row.date),
            Cell::new(&row.repo),
            Cell::new(&row.sha_short),
            Cell::new(&row.category),
            Cell::new(&row.message),
        ]);
    }

    println!("{table}");
}
