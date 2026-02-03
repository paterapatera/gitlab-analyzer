//! コマンド: プロジェクトビュー月次集計
//!
//! 特定プロジェクト/ブランチの月次集計を返す。

use crate::error::AppResult;
use crate::stats::{aggregate_project_view, MonthlyStatsResponse};
use crate::storage::CommitRepository;
use serde::Deserialize;
use tracing::info;

/// プロジェクトビュー集計リクエスト
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectViewStatsRequest {
    /// プロジェクト ID
    pub project_id: i64,
    /// ブランチ名
    pub branch_name: String,
    /// 対象年
    pub year: i32,
    /// フィルタするユーザーキー
    #[serde(default)]
    pub user_keys: Vec<String>,
}

/// プロジェクトビューの月次集計
#[tauri::command]
pub fn get_monthly_stats_project_view(request: ProjectViewStatsRequest) -> Result<MonthlyStatsResponse, String> {
    get_monthly_stats_project_view_inner(request)
        .map_err(|e| e.to_string())
}

fn get_monthly_stats_project_view_inner(request: ProjectViewStatsRequest) -> AppResult<MonthlyStatsResponse> {
    info!(
        "プロジェクトビュー集計: project_id={}, branch={}, year={}",
        request.project_id,
        request.branch_name,
        request.year
    );
    
    // コミットを取得
    let commits = CommitRepository::find_by_project_and_branch(
        request.project_id,
        &request.branch_name,
    )?;
    
    // 年でフィルタ
    let commits: Vec<_> = commits
        .into_iter()
        .filter(|c| c.year() == request.year)
        .collect();
    
    info!("集計対象コミット数: {}", commits.len());
    
    // 集計
    let response = aggregate_project_view(&commits, &request.user_keys);
    
    Ok(response)
}
