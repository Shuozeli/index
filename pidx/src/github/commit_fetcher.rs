use anyhow::Context;

use super::GithubClient;
use super::types::GithubCommit;

impl GithubClient {
    pub async fn fetch_commits(
        &self,
        repo: &str,
        per_page: u32,
    ) -> anyhow::Result<Vec<GithubCommit>> {
        let url = self.repo_url(repo, "commits");
        let resp = self
            .client()
            .get(&url)
            .query(&[("per_page", per_page.to_string())])
            .send()
            .await
            .context("Failed to fetch commits")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error {status} for commits on {repo}: {body}");
        }

        let commits: Vec<GithubCommit> = resp
            .json()
            .await
            .context("Failed to parse commits response")?;
        Ok(commits)
    }
}
