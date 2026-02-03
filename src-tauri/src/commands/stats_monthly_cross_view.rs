//! コマンド: 横断ビュー月次集計
//!
//! 全プロジェクト横断の月次集計を返す。

use crate::error::AppResult;
use crate::stats::{aggregate_cross_view, MonthlyStatsResponse};
use crate::storage::CommitRepository;
use serde::Deserialize;
use tracing::info;

/// 横断ビュー集計リクエスト
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrossViewStatsRequest {
    /// 対象年
    pub year: i32,
    /// フィルタするユーザーキー
    #[serde(default)]
    pub user_keys: Vec<String>,
}

/// 横断ビューの月次集計
#[tauri::command]
pub fn get_monthly_stats_cross_view(request: CrossViewStatsRequest) -> Result<MonthlyStatsResponse, String> {
    get_monthly_stats_cross_view_inner(request)
        .map_err(|e| e.to_string())
}

fn get_monthly_stats_cross_view_inner(request: CrossViewStatsRequest) -> AppResult<MonthlyStatsResponse> {
    info!("横断ビュー集計: year={}", request.year);
    
    // 年でフィルタしたコミットを取得
    let commits = CommitRepository::find_by_year(request.year)?;
    
    info!("集計対象コミット数: {}", commits.len());
    
    // 集計
    let response = aggregate_cross_view(&commits, &request.user_keys);
    
    Ok(response)
}
