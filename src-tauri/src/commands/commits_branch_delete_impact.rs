//! コマンド: ブランチ削除影響サマリ取得
//!
//! 対象プロジェクト/ブランチの削除影響件数を算出する。

use crate::error::AppResult;
use crate::storage;
use serde::{Deserialize, Serialize};
use tracing::info;

/// 影響サマリリクエスト
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteBranchImpactRequest {
    /// プロジェクト ID
    pub project_id: i64,
    /// ブランチ名
    pub branch_name: String,
}

/// 影響を受けるビュー種別
#[derive(Debug, Clone, Serialize)]
pub enum AffectedView {
    /// プロジェクトビュー
    #[serde(rename = "project-view")]
    ProjectView,
    /// 横断ビュー
    #[serde(rename = "cross-view")]
    CrossView,
}

/// 影響サマリレスポンス
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteBranchImpactResponse {
    /// プロジェクト ID
    pub project_id: i64,
    /// ブランチ名
    pub branch_name: String,
    /// 削除対象コミット数
    pub commit_count: i64,
    /// 影響を受けるビュー
    pub affected_views: Vec<AffectedView>,
    /// 削除可能かどうか
    pub can_delete: bool,
    /// ブロック理由（削除不可の場合）
    pub block_reason: Option<String>,
    /// ステータス
    pub status: String,
}

/// ブランチ削除の影響サマリを取得
#[tauri::command]
pub async fn get_branch_delete_impact(
    request: DeleteBranchImpactRequest,
) -> Result<DeleteBranchImpactResponse, String> {
    get_branch_delete_impact_inner(request).map_err(|e| e.user_message())
}

/// 影響サマリ取得の内部実装
fn get_branch_delete_impact_inner(
    request: DeleteBranchImpactRequest,
) -> AppResult<DeleteBranchImpactResponse> {
    info!(
        "影響サマリ取得: project_id={}, branch={}",
        request.project_id, request.branch_name
    );

    // 収集中チェック
    let is_collecting = storage::bulk_collection_repository::is_branch_collecting(
        request.project_id,
        &request.branch_name,
    )?;

    if is_collecting {
        info!(
            "収集中のためブロック: project_id={}, branch={}",
            request.project_id, request.branch_name
        );
        return Ok(DeleteBranchImpactResponse {
            project_id: request.project_id,
            branch_name: request.branch_name,
            commit_count: 0,
            affected_views: Vec::new(),
            can_delete: false,
            block_reason: Some(
                "このブランチは現在収集中です。収集完了後に再度お試しください。".to_string(),
            ),
            status: "blocked".to_string(),
        });
    }

    // コミット件数を取得
    let commit_count =
        storage::CommitRepository::count_by_branch(request.project_id, &request.branch_name)?;

    if commit_count == 0 {
        info!(
            "コミットなし: project_id={}, branch={}",
            request.project_id, request.branch_name
        );
        return Ok(DeleteBranchImpactResponse {
            project_id: request.project_id,
            branch_name: request.branch_name,
            commit_count: 0,
            affected_views: Vec::new(),
            can_delete: false,
            block_reason: None,
            status: "no_commits".to_string(),
        });
    }

    // 影響ビューは常に both（プロジェクトビューと横断ビュー）
    let affected_views = vec![AffectedView::ProjectView, AffectedView::CrossView];

    info!(
        "影響サマリ: project_id={}, branch={}, commits={}",
        request.project_id, request.branch_name, commit_count
    );

    Ok(DeleteBranchImpactResponse {
        project_id: request.project_id,
        branch_name: request.branch_name,
        commit_count,
        affected_views,
        can_delete: true,
        block_reason: None,
        status: "ok".to_string(),
    })
}
