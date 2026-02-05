/// ユーザーフィルタの選択状態を管理する SQLite リポジトリ
///
/// 統計表示時のユーザーフィルタ選択状態を永続化します。

use anyhow::{Context, Result};
use rusqlite::Connection;

pub struct UserFilterRepository;

impl UserFilterRepository {
    /// ユーザーフィルタを保存
    pub fn set_user_filter(
        conn: &Connection,
        view_type: &str, // "project-view" | "cross-view"
        context_key: &str, // プロジェクト ID や期間など
        selected_users: Vec<String>,
    ) -> Result<()> {
        let selected_users_json = serde_json::to_string(&selected_users)
            .context("Failed to serialize selected users")?;
        let updated_at = chrono::Utc::now().to_rfc3339();
        
        // 既存レコードを削除
        conn.execute(
            "DELETE FROM user_filters WHERE view_type = ? AND context_key = ?",
            rusqlite::params![view_type, context_key],
        ).context("Failed to delete existing filter")?;
        
        // 新しいフィルタを挿入
        conn.execute(
            "INSERT INTO user_filters (view_type, context_key, selected_users_json, updated_at_utc)
             VALUES (?, ?, ?, ?)",
            rusqlite::params![view_type, context_key, selected_users_json, updated_at],
        ).context("Failed to insert user filter")?;
        
        tracing::debug!("User filter saved for view: {}, context: {}", view_type, context_key);
        Ok(())
    }
    
    /// ユーザーフィルタを取得
    pub fn get_user_filter(
        conn: &Connection,
        view_type: &str,
        context_key: &str,
    ) -> Result<Option<Vec<String>>> {
        let mut stmt = conn.prepare(
            "SELECT selected_users_json FROM user_filters WHERE view_type = ? AND context_key = ?"
        ).context("Failed to prepare user filter query")?;
        
        let result = stmt.query_row(rusqlite::params![view_type, context_key], |row| {
            let json_str: String = row.get(0)?;
            let users = serde_json::from_str::<Vec<String>>(&json_str)
                .map_err(|_| rusqlite::Error::InvalidQuery)?;
            Ok(users)
        });
        
        match result {
            Ok(users) => {
                tracing::debug!("Retrieved user filter for view: {}, context: {}", view_type, context_key);
                Ok(Some(users))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                tracing::debug!("No user filter found for view: {}, context: {}", view_type, context_key);
                Ok(None)
            }
            Err(e) => Err(anyhow::anyhow!("Failed to query user filter: {}", e)),
        }
    }
    
    /// ユーザーフィルタを削除
    pub fn delete_user_filter(
        conn: &Connection,
        view_type: &str,
        context_key: &str,
    ) -> Result<()> {
        conn.execute(
            "DELETE FROM user_filters WHERE view_type = ? AND context_key = ?",
            rusqlite::params![view_type, context_key],
        ).context("Failed to delete user filter")?;
        
        tracing::debug!("User filter deleted for view: {}, context: {}", view_type, context_key);
        Ok(())
    }
    
    /// ビュータイプ全体のフィルタを削除
    pub fn delete_all_filters_by_view(
        conn: &Connection,
        view_type: &str,
    ) -> Result<()> {
        conn.execute(
            "DELETE FROM user_filters WHERE view_type = ?",
            rusqlite::params![view_type],
        ).context("Failed to delete view filters")?;
        
        tracing::info!("All filters for view '{}' deleted", view_type);
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
            "CREATE TABLE user_filters (
                view_type TEXT NOT NULL,
                context_key TEXT NOT NULL,
                selected_users_json TEXT NOT NULL,
                updated_at_utc TEXT NOT NULL,
                PRIMARY KEY (view_type, context_key)
            )",
            [],
        ).unwrap();
        
        conn
    }

    #[test]
    fn test_set_user_filter() {
        let conn = create_test_connection();
        
        let result = UserFilterRepository::set_user_filter(
            &conn,
            "project-view",
            "project:1",
            vec!["user1@example.com".to_string(), "user2@example.com".to_string()],
        );
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_user_filter() {
        let conn = create_test_connection();
        
        let users = vec!["user1@example.com".to_string(), "user2@example.com".to_string()];
        
        UserFilterRepository::set_user_filter(
            &conn,
            "project-view",
            "project:1",
            users.clone(),
        ).unwrap();
        
        let result = UserFilterRepository::get_user_filter(
            &conn,
            "project-view",
            "project:1",
        ).unwrap();
        
        assert!(result.is_some());
        assert_eq!(result.unwrap(), users);
    }
}
