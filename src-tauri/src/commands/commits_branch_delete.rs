//! コマンド: ブランチ単位コミット削除
//!
//! 指定プロジェクト/ブランチの収集済みコミットを物理削除する。

use crate::commands::commits_branch_delete_impact::AffectedView;
use crate::error::{AppError, AppResult};
use crate::storage;
use serde::{Deserialize, Serialize};
use tracing::info;

/// ブランチ削除リクエスト
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteBranchRequest {
    /// プロジェクト ID
    pub project_id: i64,
    /// ブランチ名
    pub branch_name: String,
}

/// ブランチ削除レスポンス
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteBranchResponse {
    /// プロジェクト ID
    pub project_id: i64,
    /// ブランチ名
    pub branch_name: String,
    /// 削除件数
    pub deleted_count: i64,
    /// 影響を受けたビュー
    pub affected_views: Vec<AffectedView>,
    /// ステータス
    pub status: String,
    /// メッセージ（省略可能）
    pub message: Option<String>,
}

/// ブランチ単位でコミットを削除
#[tauri::command]
pub async fn delete_branch_commits(
    request: DeleteBranchRequest,
) -> Result<DeleteBranchResponse, String> {
    delete_branch_commits_inner(request).map_err(|e| e.user_message())
}

/// 削除処理の内部実装
fn delete_branch_commits_inner(request: DeleteBranchRequest) -> AppResult<DeleteBranchResponse> {
    info!(
        "ブランチ削除開始: project_id={}, branch={}",
        request.project_id, request.branch_name
    );

    // 収集中チェック（FR-007: 収集中の同一ブランチは削除不可）
    let is_collecting = storage::bulk_collection_repository::is_branch_collecting(
        request.project_id,
        &request.branch_name,
    )?;

    if is_collecting {
        return Err(AppError::Validation(
            "このブランチは現在収集中です。収集完了後に再度お試しください。".to_string(),
        ));
    }

    // コミット件数を確認
    let commit_count =
        storage::CommitRepository::count_by_branch(request.project_id, &request.branch_name)?;

    if commit_count == 0 {
        info!(
            "削除対象なし: project_id={}, branch={}",
            request.project_id, request.branch_name
        );
        return Ok(DeleteBranchResponse {
            project_id: request.project_id,
            branch_name: request.branch_name,
            deleted_count: 0,
            affected_views: Vec::new(),
            status: "no_commits".to_string(),
            message: Some("削除対象のコミットがありません。".to_string()),
        });
    }

    // 物理削除（FR-008: 復元不可）
    let deleted_count =
        storage::CommitRepository::delete_by_branch(request.project_id, &request.branch_name)?;

    let affected_views = vec![AffectedView::ProjectView, AffectedView::CrossView];

    info!(
        "ブランチ削除完了: project_id={}, branch={}, deleted={}",
        request.project_id, request.branch_name, deleted_count
    );

    Ok(DeleteBranchResponse {
        project_id: request.project_id,
        branch_name: request.branch_name,
        deleted_count,
        affected_views,
        status: "deleted".to_string(),
        message: None,
    })
}
