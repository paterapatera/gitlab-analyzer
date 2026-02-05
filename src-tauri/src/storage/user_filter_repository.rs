//! ユーザーフィルタリポジトリ（SQLite ベース）
//!
//! ユーザーフィルタ選択状態を SQLite で永続化します。

use crate::error::{AppError, AppResult};
use crate::storage::sqlite;

/// ユーザーフィルタ選択状態のビュー種別
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserFilterViewType {
    /// プロジェクト別ビュー
    ProjectView,
    /// 横断ビュー
    CrossView,
}

impl UserFilterViewType {
    /// ビュータイプを文字列に変換
    pub fn as_str(&self) -> &str {
        match self {
            Self::ProjectView => "project-view",
            Self::CrossView => "cross-view",
        }
    }
}

/// コンテキストキー（プロジェクト/ブランチ/年 or 年のみ）
pub type UserFilterContextKey = String;

/// 選択されたユーザーID配列
pub type SelectedUsers = Vec<String>;

/// ユーザーフィルタ選択状態リポジトリ
pub struct UserFilterRepository;

impl UserFilterRepository {
    /// 選択状態を取得
    pub fn get(view_type: &UserFilterViewType, context_key: &str) -> AppResult<SelectedUsers> {
        let conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        let result =
            sqlite::UserFilterRepository::get_user_filter(&conn, view_type.as_str(), context_key)
                .map_err(|e| AppError::Storage(e.to_string()))?;

        Ok(result.unwrap_or_default())
    }

    /// 選択状態を保存
    pub fn set(
        view_type: &UserFilterViewType,
        context_key: &str,
        selected_users: SelectedUsers,
    ) -> AppResult<()> {
        let conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        sqlite::UserFilterRepository::set_user_filter(
            &conn,
            view_type.as_str(),
            context_key,
            selected_users,
        )
        .map_err(|e| AppError::Storage(e.to_string()))?;

        Ok(())
    }
}
