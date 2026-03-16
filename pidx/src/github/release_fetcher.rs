use anyhow::Context;

use super::GithubClient;
use super::types::GithubRelease;

impl GithubClient {
    pub async fn fetch_releases(&self, repo: &str) -> anyhow::Result<Vec<GithubRelease>> {
        let url = self.repo_url(repo, "releases");
        let resp = self
            .client()
            .get(&url)
            .query(&[("per_page", "20")])
            .send()
            .await
            .context("Failed to fetch releases")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error {status} for releases on {repo}: {body}");
        }

        let releases: Vec<GithubRelease> = resp
            .json()
            .await
            .context("Failed to parse releases response")?;
        Ok(releases)
    }
}
