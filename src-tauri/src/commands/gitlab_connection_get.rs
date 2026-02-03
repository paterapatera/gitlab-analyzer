//! コマンド: 接続設定取得
//!
//! 登録済みの GitLab 接続情報を取得する。
//! セキュリティ要件により、アクセストークンは返却しない（FR-016）。

use crate::domain::GitLabConnectionPublic;
use crate::error::AppResult;
use crate::storage::ConnectionRepository;

/// 接続設定を取得（トークン非返却）
#[tauri::command]
pub fn get_gitlab_connection() -> Result<Option<GitLabConnectionPublic>, String> {
    get_gitlab_connection_inner()
        .map_err(|e| e.to_string())
}

fn get_gitlab_connection_inner() -> AppResult<Option<GitLabConnectionPublic>> {
    let conn = ConnectionRepository::get()?;
    Ok(conn.as_ref().map(GitLabConnectionPublic::from))
}
