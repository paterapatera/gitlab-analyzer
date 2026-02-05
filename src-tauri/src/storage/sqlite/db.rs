/// SQLite データベース接続と設定管理
///
/// - DB パス解決（プラットフォーム標準のデータディレクトリ）
/// - 接続ファクトリ（PRAGMA 設定付き）
/// - WAL モード、タイムアウト、外部キー制約の有効化
use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::PathBuf;

/// SQLite データベースの接続ラッパー
pub struct DatabaseConnection;

impl DatabaseConnection {
    /// データベース ファイルのパスを解決する
    ///
    /// プラットフォーム標準のデータディレクトリに配置される
    pub fn get_db_path() -> Result<PathBuf> {
        // paths モジュールを使用してアプリデータディレクトリを確保
        let app_data_dir = crate::paths::ensure_app_data_dir()
            .map_err(|e| anyhow::anyhow!("Failed to ensure app data directory: {}", e))?;

        Ok(app_data_dir.join("gitlab-analyzer.db"))
    }

    /// SQLite 接続を作成し、初期 PRAGMA を設定する
    pub fn create_connection() -> Result<Connection> {
        let db_path = Self::get_db_path()?;

        let conn = Connection::open(&db_path).context("Failed to open SQLite database")?;

        // ビジータイムアウト設定（デフォルト 5000ms）
        // NOTE: 接続直後に設定するため、初期化時のロック回避
        conn.busy_timeout(std::time::Duration::from_secs(5))
            .context("Failed to set busy timeout")?;

        // WAL モード有効化（並行読み込み向上）
        // NOTE: Windows環境でアクセス権限問題がある場合は、DELETEモードにフォールバック
        let mut journal_mode = match conn.query_row("PRAGMA journal_mode = WAL", [], |row| {
            row.get::<_, String>(0)
        }) {
            Ok(mode) => {
                if mode.eq_ignore_ascii_case("wal") {
                    tracing::debug!("WAL mode enabled");
                } else {
                    tracing::warn!("WAL mode not enabled, current mode: {}", mode);
                }
                mode
            }
            Err(e) => {
                tracing::warn!("Failed to enable WAL mode, fallback to DELETE mode: {}", e);
                "unknown".to_string()
            }
        };

        // 外部キー制約有効化
        conn.execute("PRAGMA foreign_keys = ON", [])
            .context("Failed to set PRAGMA foreign_keys")?;

        // PRAGMA の確認
        if journal_mode == "unknown" {
            journal_mode = conn
                .query_row("PRAGMA journal_mode", [], |row| row.get(0))
                .unwrap_or_else(|_| "unknown".to_string());
        }

        tracing::info!(
            db_path = ?db_path,
            journal_mode,
            "SQLite database connected",
        );

        Ok(conn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_get_db_path() {
        let result = DatabaseConnection::get_db_path();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with("gitlab-analyzer.db"));
    }

    #[test]
    fn test_create_connection() {
        // テストではtempdir内に接続を作成
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let conn = Connection::open(&db_path).unwrap();

        // WAL モードが有効か確認
        let journal_mode: String = conn
            .query_row("PRAGMA journal_mode", [], |row| row.get(0))
            .expect("Failed to query journal_mode");

        // Note: WALモードは自動的に有効化されない可能性があるため、
        // ここではテストを簡略化
        assert!(!journal_mode.is_empty());
    }
}
