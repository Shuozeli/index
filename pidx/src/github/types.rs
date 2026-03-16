use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GithubRepo {
    pub name: String,
    pub language: Option<String>,
    pub description: Option<String>,
    pub open_issues_count: i32,
    pub pushed_at: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GithubCommit {
    pub sha: String,
    pub commit: GithubCommitDetail,
    pub author: Option<GithubUser>,
}

#[derive(Debug, Deserialize)]
pub struct GithubCommitDetail {
    pub message: String,
    pub author: Option<GithubCommitAuthor>,
}

#[derive(Debug, Deserialize)]
pub struct GithubCommitAuthor {
    pub name: Option<String>,
    pub date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GithubUser {
    pub login: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubIssue {
    pub number: i32,
    pub title: String,
    pub state: String,
    pub labels: Vec<GithubLabel>,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub closed_at: Option<String>,
    pub pull_request: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct GithubLabel {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubRelease {
    pub tag_name: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub published_at: Option<String>,
}
