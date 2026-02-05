//! 接続設定リポジトリ（SQLite ベース）
//!
//! GitLabConnection の永続化を SQLite で行います。

use crate::domain::GitLabConnection;
use crate::error::{AppError, AppResult};
use crate::storage::sqlite;

/// 接続設定リポジトリ
pub struct ConnectionRepository;

impl ConnectionRepository {
    /// 接続設定を取得
    pub fn get() -> AppResult<Option<GitLabConnection>> {
        let conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        let result = sqlite::ConnectionRepository::get_connection(&conn)
            .map_err(|e| AppError::Storage(e.to_string()))?;

        if let Some((base_url, _author_email, access_token)) = result {
            if access_token.is_empty() {
                tracing::warn!("Access token not found in database");
                return Ok(None);
            }

            let connection = GitLabConnection {
                base_url,
                access_token,
                updated_at_utc: chrono::Utc::now(),
            };

            Ok(Some(connection))
        } else {
            Ok(None)
        }
    }

    /// 接続設定を保存
    pub fn save(connection: GitLabConnection) -> AppResult<()> {
        let conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        sqlite::ConnectionRepository::set_connection(
            &conn,
            connection.base_url,
            None, // author_email は接続設定に含まない
            connection.access_token,
        )
        .map_err(|e| AppError::Storage(e.to_string()))?;

        Ok(())
    }
}
