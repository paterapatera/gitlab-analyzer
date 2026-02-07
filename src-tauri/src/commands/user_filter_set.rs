//! ユーザーフィルタ選択状態保存コマンド
//!
//! 指定されたビュー種別とコンテキストキーに対応する選択状態を保存する。

use crate::storage::user_filter_repository::{
    SelectedUsers, UserFilterRepository, UserFilterViewType,
};

/// ユーザーフィルタ選択状態を保存
///
/// # Arguments
/// * `view_type` - ビュー種別（"project-view" or "cross-view"）
/// * `context_key` - コンテキストキー（プロジェクト/ブランチ/年 or 年のみ）
/// * `selected_users` - 選択されたユーザーID配列
///
/// NOTE: 同一のview_type+context_keyが存在する場合は上書き
/// NOTE: 空配列は「全解除」状態を表す
#[tauri::command]
pub async fn user_filter_set(
    view_type: String,
    context_key: String,
    selected_users: SelectedUsers,
) -> Result<(), String> {
    let view_type = parse_view_type(&view_type)?;

    UserFilterRepository::set(&view_type, &context_key, selected_users)
        .map_err(|e| format!("フィルタ状態の保存に失敗しました: {}", e))
}

/// view_type文字列をenumにパース
fn parse_view_type(view_type: &str) -> Result<UserFilterViewType, String> {
    match view_type {
        "project-view" => Ok(UserFilterViewType::ProjectView),
        "cross-view" => Ok(UserFilterViewType::CrossView),
        _ => Err(format!("無効なビュー種別です: {}", view_type)),
    }
}
