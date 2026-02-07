/// GitLab 接続設定の SQLite リポジトリ
///
/// 接続設定（base_url、author_email、access_token）を SQLite に保存します。
use anyhow::{Context, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabConnectionRecord {
    pub base_url: String,
    pub author_email: Option<String>,
    pub access_token: String,
    pub updated_at_utc: String,
}

pub struct ConnectionRepository;

impl ConnectionRepository {
    /// 接続設定をデータベースに保存（1レコードのみ）
    pub fn set_connection(
        conn: &Connection,
        base_url: String,
        author_email: Option<String>,
        access_token: String,
    ) -> Result<()> {
        let updated_at = chrono::Utc::now().to_rfc3339();

        // 既存レコードを削除（1レコードのみ）
        conn.execute("DELETE FROM connections", [])
            .context("Failed to delete existing connection")?;

        // 新しい接続情報を挿入
        conn.execute(
            "INSERT INTO connections (base_url, author_email, access_token, updated_at_utc)
             VALUES (?, ?, ?, ?)",
            rusqlite::params![&base_url, author_email, access_token, updated_at],
        )
        .context("Failed to insert connection record")?;

        tracing::info!("GitLab connection settings saved");
        Ok(())
    }

    /// 接続設定をデータベースから取得
    pub fn get_connection(conn: &Connection) -> Result<Option<(String, Option<String>, String)>> {
        let mut stmt = conn
            .prepare("SELECT base_url, author_email, access_token FROM connections LIMIT 1")
            .context("Failed to prepare connection query")?;

        let result = stmt.query_row([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, Option<String>>(1)?,
                row.get::<_, String>(2)?,
            ))
        });

        match result {
            Ok(values) => {
                tracing::debug!("Retrieved GitLab connection settings");
                Ok(Some(values))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                tracing::debug!("No GitLab connection settings found");
                Ok(None)
            }
            Err(e) => Err(anyhow::anyhow!("Failed to query connection: {}", e)),
        }
    }

    /// 接続設定を削除
    pub fn delete_connection(conn: &Connection) -> Result<()> {
        // データベースから接続情報を削除
        conn.execute("DELETE FROM connections", [])
            .context("Failed to delete connection record")?;

        tracing::info!("GitLab connection settings deleted");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_connection() -> rusqlite::Connection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();

        // テーブル作成
        conn.execute(
            "CREATE TABLE connections (
                id INTEGER PRIMARY KEY DEFAULT 1,
                base_url TEXT NOT NULL,
                author_email TEXT,
                access_token TEXT NOT NULL,
                updated_at_utc TEXT NOT NULL
            )",
            [],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_set_connection() {
        let conn = create_test_connection();

        let result = ConnectionRepository::set_connection(
            &conn,
            "https://gitlab.example.com".to_string(),
            Some("user@example.com".to_string()),
            "test_token_123".to_string(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_get_connection() {
        let conn = create_test_connection();

        ConnectionRepository::set_connection(
            &conn,
            "https://gitlab.example.com".to_string(),
            Some("user@example.com".to_string()),
            "test_token_123".to_string(),
        )
        .unwrap();

        let result = ConnectionRepository::get_connection(&conn).unwrap();
        assert!(result.is_some());

        let (base_url, author_email, access_token) = result.unwrap();
        assert_eq!(base_url, "https://gitlab.example.com");
        assert_eq!(author_email, Some("user@example.com".to_string()));
        assert_eq!(access_token, "test_token_123");
    }
}
