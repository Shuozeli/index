pub mod commit_store;
pub mod issue_store;
pub mod llm_summary_store;
pub mod release_store;
pub mod repo_store;
pub mod schema;
pub mod sync_log_store;

use std::path::Path;

use anyhow::Context;
use rusqlite::Connection;

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create database directory")?;
        }
        let conn = Connection::open(path).context("Failed to open database")?;
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON;")
            .context("Failed to set pragmas")?;
        let db = Database { conn };
        db.migrate()?;
        Ok(db)
    }

    fn migrate(&self) -> anyhow::Result<()> {
        self.conn
            .execute_batch(schema::CREATE_TABLES)
            .context("Failed to run migrations")?;
        Ok(())
    }

    /// Execute a closure within a transaction. All DB access goes through here.
    pub fn tx<F, T>(&self, f: F) -> anyhow::Result<T>
    where
        F: FnOnce(&Connection) -> anyhow::Result<T>,
    {
        let tx = self
            .conn
            .unchecked_transaction()
            .context("Failed to begin transaction")?;
        let result = f(&tx)?;
        tx.commit().context("Failed to commit transaction")?;
        Ok(result)
    }
}
