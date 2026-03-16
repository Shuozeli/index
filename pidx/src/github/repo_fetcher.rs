use anyhow::Context;

use super::GithubClient;
use super::types::GithubRepo;

impl GithubClient {
    pub async fn fetch_repo(&self, repo: &str) -> anyhow::Result<GithubRepo> {
        let url = format!(
            "https://api.github.com/repos/{}/{}",
            self.owner(),
            repo
        );
        let resp = self
            .client()
            .get(&url)
            .send()
            .await
            .context("Failed to fetch repo")?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("GitHub API error {status} for repo {repo}: {body}");
        }

        let github_repo: GithubRepo = resp
            .json()
            .await
            .context("Failed to parse repo response")?;
        Ok(github_repo)
    }
}
