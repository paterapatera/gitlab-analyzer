//! ユーザーフィルタ選択状態リポジトリ
//!
//! ユーザーフィルタリングの選択状態を永続化する。
//! プロジェクト別ビューと横断ビューで独立した選択状態を管理。

use crate::error::AppResult;
use crate::storage::{read_json, write_json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// ストレージファイル名
const USER_FILTER_FILE: &str = "user_filter_state.json";

/// ユーザーフィルタ選択状態のビュー種別
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UserFilterViewType {
    /// プロジェクト別ビュー
    ProjectView,
    /// 横断ビュー
    CrossView,
}

/// コンテキストキー（プロジェクト/ブランチ/年 or 年のみ）
pub type UserFilterContextKey = String;

/// 選択されたユーザーID配列
pub type SelectedUsers = Vec<String>;

/// ユーザーフィルタ選択状態全体のストレージ構造
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserFilterStorage {
    /// プロジェクト別ビューの選択状態
    #[serde(rename = "project-view", default)]
    pub project_view: HashMap<UserFilterContextKey, SelectedUsers>,
    /// 横断ビューの選択状態
    #[serde(rename = "cross-view", default)]
    pub cross_view: HashMap<UserFilterContextKey, SelectedUsers>,
}

/// ユーザーフィルタ選択状態リポジトリ
pub struct UserFilterRepository;

impl UserFilterRepository {
    /// 選択状態を取得
    ///
    /// # Arguments
    /// * `view_type` - ビュー種別
    /// * `context_key` - コンテキストキー
    ///
    /// # Returns
    /// 選択されたユーザーID配列（存在しない場合は空配列）
    ///
    /// NOTE: 空配列は「未保存（全選択として扱う）」を意味する
    pub fn get(view_type: &UserFilterViewType, context_key: &str) -> AppResult<SelectedUsers> {
        let storage = read_json::<UserFilterStorage>(USER_FILTER_FILE)?
            .unwrap_or_default();
        
        let map = match view_type {
            UserFilterViewType::ProjectView => &storage.project_view,
            UserFilterViewType::CrossView => &storage.cross_view,
        };
        
        Ok(map.get(context_key).cloned().unwrap_or_default())
    }
    
    /// 選択状態を保存
    ///
    /// # Arguments
    /// * `view_type` - ビュー種別
    /// * `context_key` - コンテキストキー
    /// * `selected_users` - 選択されたユーザーID配列
    ///
    /// NOTE: 同一のview_type+context_keyが存在する場合は上書き
    pub fn set(
        view_type: &UserFilterViewType,
        context_key: &str,
        selected_users: SelectedUsers,
    ) -> AppResult<()> {
        let mut storage = read_json::<UserFilterStorage>(USER_FILTER_FILE)?
            .unwrap_or_default();
        
        let map = match view_type {
            UserFilterViewType::ProjectView => &mut storage.project_view,
            UserFilterViewType::CrossView => &mut storage.cross_view,
        };
        
        map.insert(context_key.to_string(), selected_users);
        write_json(USER_FILTER_FILE, &storage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::get_data_file_path;
    use std::fs;
    use serial_test::serial;

    fn cleanup_test_data() {
        if let Ok(path) = get_data_file_path(USER_FILTER_FILE) {
            let _ = fs::remove_file(path);
        }
    }

    #[test]
    #[serial]
    fn test_get_nonexistent() {
        cleanup_test_data();
        
        let result = UserFilterRepository::get(
            &UserFilterViewType::ProjectView,
            "test-project/main/2025"
        );
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
        
        cleanup_test_data();
    }

    #[test]
    #[serial]
    fn test_set_and_get_project_view() {
        cleanup_test_data();
        
        let users = vec!["user1@example.com".to_string(), "user2@example.com".to_string()];
        let context_key = "my-project/main/2025";
        
        let set_result = UserFilterRepository::set(
            &UserFilterViewType::ProjectView,
            context_key,
            users.clone()
        );
        assert!(set_result.is_ok());
        
        let get_result = UserFilterRepository::get(
            &UserFilterViewType::ProjectView,
            context_key
        );
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap(), users);
        
        cleanup_test_data();
    }

    #[test]
    #[serial]
    fn test_set_and_get_cross_view() {
        cleanup_test_data();
        
        let users = vec!["user1@example.com".to_string()];
        let context_key = "2025";
        
        let set_result = UserFilterRepository::set(
            &UserFilterViewType::CrossView,
            context_key,
            users.clone()
        );
        assert!(set_result.is_ok());
        
        let get_result = UserFilterRepository::get(
            &UserFilterViewType::CrossView,
            context_key
        );
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap(), users);
        
        cleanup_test_data();
    }

    #[test]
    #[serial]
    fn test_overwrite_existing() {
        cleanup_test_data();
        
        let context_key = "test-project/develop/2024";
        
        // 最初の保存
        let users1 = vec!["user1@example.com".to_string()];
        UserFilterRepository::set(
            &UserFilterViewType::ProjectView,
            context_key,
            users1
        ).unwrap();
        
        // 上書き保存
        let users2 = vec!["user2@example.com".to_string(), "user3@example.com".to_string()];
        UserFilterRepository::set(
            &UserFilterViewType::ProjectView,
            context_key,
            users2.clone()
        ).unwrap();
        
        // 上書き確認
        let result = UserFilterRepository::get(
            &UserFilterViewType::ProjectView,
            context_key
        ).unwrap();
        assert_eq!(result, users2);
        
        cleanup_test_data();
    }

    #[test]
    #[serial]
    fn test_view_type_independence() {
        cleanup_test_data();
        
        let context_key = "2025";
        
        // プロジェクト別ビューに保存
        let project_users = vec!["project-user@example.com".to_string()];
        UserFilterRepository::set(
            &UserFilterViewType::ProjectView,
            context_key,
            project_users.clone()
        ).unwrap();
        
        // 横断ビューに保存
        let cross_users = vec!["cross-user@example.com".to_string()];
        UserFilterRepository::set(
            &UserFilterViewType::CrossView,
            context_key,
            cross_users.clone()
        ).unwrap();
        
        // それぞれ独立して取得できる
        let project_result = UserFilterRepository::get(
            &UserFilterViewType::ProjectView,
            context_key
        ).unwrap();
        let cross_result = UserFilterRepository::get(
            &UserFilterViewType::CrossView,
            context_key
        ).unwrap();
        
        assert_eq!(project_result, project_users);
        assert_eq!(cross_result, cross_users);
        
        cleanup_test_data();
    }

    #[test]
    #[serial]
    fn test_empty_array_for_deselect_all() {
        cleanup_test_data();
        
        let context_key = "test/main/2025";
        
        // 空配列を保存（全解除）
        UserFilterRepository::set(
            &UserFilterViewType::ProjectView,
            context_key,
            vec![]
        ).unwrap();
        
        // 空配列として取得できる
        let result = UserFilterRepository::get(
            &UserFilterViewType::ProjectView,
            context_key
        ).unwrap();
        assert!(result.is_empty());
        
        cleanup_test_data();
    }
}
