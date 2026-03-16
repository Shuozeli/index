use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommitCategory {
    Feature,
    Bugfix,
    Docs,
    Refactor,
    Test,
    Sync,
    Chore,
}

impl CommitCategory {
    pub fn from_message(message: &str) -> Self {
        let lower = message.to_lowercase();
        if lower.starts_with("fix:") || lower.starts_with("fix(") {
            CommitCategory::Bugfix
        } else if lower.starts_with("feat:")
            || lower.starts_with("feat(")
            || lower.starts_with("add ")
        {
            CommitCategory::Feature
        } else if lower.starts_with("docs:") || lower.starts_with("docs(") {
            CommitCategory::Docs
        } else if lower.starts_with("refactor:") || lower.starts_with("refactor(") {
            CommitCategory::Refactor
        } else if lower.starts_with("test:") || lower.starts_with("test(") {
            CommitCategory::Test
        } else if lower.starts_with("sync:") || lower.starts_with("sync(") {
            CommitCategory::Sync
        } else {
            CommitCategory::Chore
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            CommitCategory::Feature => "feature",
            CommitCategory::Bugfix => "bugfix",
            CommitCategory::Docs => "docs",
            CommitCategory::Refactor => "refactor",
            CommitCategory::Test => "test",
            CommitCategory::Sync => "sync",
            CommitCategory::Chore => "chore",
        }
    }
}

impl fmt::Display for CommitCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
