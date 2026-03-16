use anyhow::Context;

use super::GithubClient;
use super::types::GithubIssue;

impl GithubClient {
    pub async fn fetch_issues(
        &self,
        repo: &str,
        state: &str,
    ) -> anyhow::Result<Vec<GithubIssue>> {
        let url = self.repo_url(repo, "issues");
        let resp = self
            .client()
            .get(&url)
            .query(&[
                ("state", state),
                ("per_page", "100"),
            ])
            .send()
            .await
            .context("Failed to fetch issues")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error {status} for issues on {repo}: {body}");
        }

        let issues: Vec<GithubIssue> = resp
            .json()
            .await
            .context("Failed to parse issues response")?;

        // Filter out pull requests (GitHub API returns PRs as issues)
        let issues = issues
            .into_iter()
            .filter(|i| i.pull_request.is_none())
            .collect();

        Ok(issues)
    }
}
