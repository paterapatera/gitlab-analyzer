//! ユーザーフィルタ選択状態取得コマンド
//!
//! 指定されたビュー種別とコンテキストキーに対応する選択状態を取得する。

use crate::storage::user_filter_repository::{
    SelectedUsers, UserFilterRepository, UserFilterViewType,
};

/// ユーザーフィルタ選択状態を取得
///
/// # Arguments
/// * `view_type` - ビュー種別（"project-view" or "cross-view"）
/// * `context_key` - コンテキストキー（プロジェクト/ブランチ/年 or 年のみ）
///
/// # Returns
/// 選択されたユーザーID配列（存在しない場合は空配列）
///
/// NOTE: 空配列は「未保存（全選択として扱う）」を意味し、
/// フロントエンド側で全ユーザー選択として解釈する
#[tauri::command]
pub async fn user_filter_get(
    view_type: String,
    context_key: String,
) -> Result<SelectedUsers, String> {
    let view_type = parse_view_type(&view_type)?;

    UserFilterRepository::get(&view_type, &context_key)
        .map_err(|e| format!("フィルタ状態の取得に失敗しました: {}", e))
}

/// view_type文字列をenumにパース
fn parse_view_type(view_type: &str) -> Result<UserFilterViewType, String> {
    match view_type {
        "project-view" => Ok(UserFilterViewType::ProjectView),
        "cross-view" => Ok(UserFilterViewType::CrossView),
        _ => Err(format!("無効なビュー種別です: {}", view_type)),
    }
}
