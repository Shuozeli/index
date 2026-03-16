use std::path::PathBuf;

use anyhow::{Context, bail};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub owner: String,
    pub index_path: Option<String>,
    pub sync: SyncConfig,
    pub repos: Vec<RepoEntry>,
    #[serde(default)]
    pub categories: Vec<CategoryEntry>,
}

#[derive(Debug, Deserialize)]
pub struct SyncConfig {
    pub github_token_env: String,
    pub commits_per_sync: u32,
    pub db_path: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepoEntry {
    pub name: String,
    pub category: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryEntry {
    pub key: String,
    pub title: String,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let path = Self::config_path();
        if !path.exists() {
            bail!(
                "Config file not found at {}. Create it manually.",
                path.display()
            );
        }
        let content =
            std::fs::read_to_string(&path).context("Failed to read config file")?;
        let config: Config =
            toml::from_str(&content).context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn config_path() -> PathBuf {
        Self::pidx_dir().join("pidx.toml")
    }

    pub fn pidx_dir() -> PathBuf {
        home_dir().join(".pidx")
    }

    pub fn db_path(&self) -> PathBuf {
        expand_tilde(&self.sync.db_path)
    }

    pub fn docs_dir() -> PathBuf {
        Self::pidx_dir().join("docs")
    }

    pub fn repo_docs_dir(repo_name: &str) -> PathBuf {
        Self::docs_dir().join(repo_name)
    }

    pub fn index_path(&self) -> anyhow::Result<PathBuf> {
        match &self.index_path {
            Some(p) => Ok(expand_tilde(p)),
            None => bail!("index_path not set in config"),
        }
    }

    pub fn github_token(&self) -> anyhow::Result<String> {
        let var_name = &self.sync.github_token_env;
        std::env::var(var_name)
            .with_context(|| format!("Environment variable {var_name} not set"))
    }
}

fn home_dir() -> PathBuf {
    std::env::var("HOME")
        .map(PathBuf::from)
        .expect("HOME environment variable not set")
}

fn expand_tilde(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/") {
        home_dir().join(rest)
    } else {
        PathBuf::from(path)
    }
}
