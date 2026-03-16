pub mod commit_fetcher;
pub mod issue_fetcher;
pub mod release_fetcher;
pub mod repo_fetcher;
pub mod types;

use anyhow::Context;
use reqwest::Client;
use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT};

pub struct GithubClient {
    client: Client,
    owner: String,
}

impl GithubClient {
    pub fn new(token: &str, owner: &str) -> anyhow::Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}"))
                .context("Invalid token format")?,
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("pidx-cli"));

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .context("Failed to build HTTP client")?;

        Ok(GithubClient {
            client,
            owner: owner.to_string(),
        })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    fn repo_url(&self, repo: &str, path: &str) -> String {
        format!(
            "https://api.github.com/repos/{}/{}/{}",
            self.owner, repo, path
        )
    }
}
