/// SQLite データベース健全性チェック
///
/// - DB ファイルサイズの監視
/// - 容量警告の実装
/// - インテグリティチェック
use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

pub const DB_SIZE_WARNING_THRESHOLD_MB: f64 = 500.0;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DatabaseHealth {
    pub file_size_mb: f64,
    pub estimated_commit_count: i32,
    pub is_integrity_ok: bool,
    pub warning: Option<String>,
}

pub struct DatabaseHealthChecker;

impl DatabaseHealthChecker {
    /// データベースの健全性をチェック
    pub fn check_database_health(conn: &Connection, db_path: &Path) -> Result<DatabaseHealth> {
        let file_size_mb = Self::get_db_file_size(db_path)?;
        let estimated_commit_count = Self::estimate_commit_count(conn)?;
        let is_integrity_ok = Self::check_integrity(conn)?;

        let warning = if file_size_mb > DB_SIZE_WARNING_THRESHOLD_MB {
            Some(format!(
                "Database size ({:.1} MB) exceeds warning threshold ({} MB). \
                 Consider archiving or removing old data.",
                file_size_mb, DB_SIZE_WARNING_THRESHOLD_MB as i32
            ))
        } else {
            None
        };

        Ok(DatabaseHealth {
            file_size_mb,
            estimated_commit_count,
            is_integrity_ok,
            warning,
        })
    }

    /// データベースファイルサイズを取得（MB 単位）
    fn get_db_file_size(db_path: &Path) -> Result<f64> {
        let metadata =
            std::fs::metadata(db_path).context("Failed to get database file metadata")?;

        let size_bytes = metadata.len();
        let size_mb = size_bytes as f64 / (1024.0 * 1024.0);

        Ok(size_mb)
    }

    /// コミット数の推定
    fn estimate_commit_count(conn: &Connection) -> Result<i32> {
        let count: i32 = conn
            .query_row("SELECT COUNT(*) FROM commits", [], |row| row.get(0))
            .unwrap_or(0);

        Ok(count)
    }

    /// インテグリティチェック
    fn check_integrity(conn: &Connection) -> Result<bool> {
        match conn.query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0)) {
            Ok(result) => {
                let is_ok = result == "ok";
                if !is_ok {
                    tracing::warn!("Database integrity check failed: {}", result);
                }
                Ok(is_ok)
            }
            Err(e) => {
                tracing::error!("Failed to run integrity check: {}", e);
                Ok(false)
            }
        }
    }

    /// データベースの VACUUM（最適化）
    pub fn vacuum_database(conn: &Connection) -> Result<()> {
        conn.execute("VACUUM", [])
            .context("Failed to vacuum database")?;

        tracing::info!("Database vacuumed successfully");
        Ok(())
    }

    /// 古いコミットを削除（日数指定）
    pub fn delete_old_commits(conn: &Connection, days_to_keep: i32) -> Result<usize> {
        let cutoff_date = format!(
            "{}",
            (chrono::Utc::now() - chrono::Duration::days(days_to_keep as i64))
                .format("%Y-%m-%dT%H:%M:%SZ")
        );

        let deleted = conn
            .execute(
                "DELETE FROM commits WHERE committed_date_utc < ?",
                rusqlite::params![cutoff_date],
            )
            .context("Failed to delete old commits")?;

        tracing::info!(
            "Deleted {} old commits (older than {} days)",
            deleted,
            days_to_keep
        );
        Ok(deleted as usize)
    }
}

pub fn check_database_health(conn: &Connection, db_path: &Path) -> Result<DatabaseHealth> {
    DatabaseHealthChecker::check_database_health(conn, db_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_connection() -> (rusqlite::Connection, std::path::PathBuf) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();

        // テーブル作成
        conn.execute(
            "CREATE TABLE commits (
                project_id INTEGER NOT NULL,
                branch_name TEXT NOT NULL,
                sha TEXT NOT NULL,
                author_name TEXT NOT NULL,
                author_email TEXT NOT NULL,
                committed_date_utc TEXT NOT NULL,
                additions INTEGER NOT NULL,
                deletions INTEGER NOT NULL,
                PRIMARY KEY (project_id, branch_name, sha)
            )",
            [],
        )
        .unwrap();

        (conn, db_path)
    }

    #[test]
    fn test_check_database_health() {
        let (conn, db_path) = create_test_connection();

        let health = DatabaseHealthChecker::check_database_health(&conn, &db_path).unwrap();

        assert!(health.file_size_mb > 0.0);
        assert_eq!(health.estimated_commit_count, 0);
        assert!(health.is_integrity_ok);
        assert!(health.warning.is_none()); // サイズが小さいので警告なし
    }
}
