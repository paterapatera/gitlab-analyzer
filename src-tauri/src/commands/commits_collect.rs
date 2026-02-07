//! コマンド: コミット収集
//!
//! 指定プロジェクト/ブランチ/期間のコミットを収集してローカルに保存する。

use crate::domain::Commit;
use crate::error::{AppError, AppResult};
use crate::gitlab::GitLabClient;
use crate::storage::{CommitRepository, ConnectionRepository};
use serde::{Deserialize, Serialize};
use tracing::info;

/// コミット収集リクエスト
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectCommitsRequest {
    /// プロジェクト ID
    pub project_id: i64,
    /// ブランチ名
    pub branch_name: String,
    /// 開始日時（ISO8601、省略可能）
    pub since_utc: Option<String>,
    /// 終了日時（ISO8601、省略可能）
    pub until_utc: Option<String>,
}

/// コミット収集結果
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectCommitsResult {
    /// 新規挿入件数
    pub inserted_count: usize,
    /// 重複スキップ件数
    pub skipped_duplicate_count: usize,
    /// stats 欠損件数
    pub missing_stats_count: usize,
}

/// コミットを収集
#[tauri::command]
pub async fn collect_commits(request: CollectCommitsRequest) -> Result<CollectCommitsResult, String> {
    collect_commits_inner(request)
        .await
        .map_err(|e| e.user_message())
}

pub(crate) async fn collect_commits_inner(
    request: CollectCommitsRequest,
) -> AppResult<CollectCommitsResult> {
    // 接続設定を取得
    let connection = ConnectionRepository::get()?
        .ok_or(AppError::ConnectionNotConfigured)?;
    
    info!(
        "コミット収集開始: project_id={}, branch={}, since={:?}, until={:?}",
        request.project_id,
        request.branch_name,
        request.since_utc,
        request.until_utc
    );
    
    // GitLab API からコミット一覧を取得
    let client = GitLabClient::new(&connection.base_url, &connection.access_token)?;
    let gitlab_commits = client.list_commits(
        request.project_id,
        &request.branch_name,
        request.since_utc.as_deref(),
        request.until_utc.as_deref(),
    ).await?;
    
    info!("取得したコミット数: {}", gitlab_commits.len());
    
    // ドメインモデルに変換
    let commits: Vec<Commit> = gitlab_commits
        .into_iter()
        .map(|c| Commit::from_gitlab(request.project_id, &request.branch_name, c))
        .collect();
    
    // stats 欠損件数をカウント
    let missing_stats_count = commits.iter().filter(|c| c.stats_missing).count();
    
    // 保存（重複スキップ）
    let upsert_result = CommitRepository::bulk_upsert(commits)?;
    
    info!(
        "コミット収集完了: inserted={}, skipped={}, missing_stats={}",
        upsert_result.inserted,
        upsert_result.skipped,
        missing_stats_count
    );
    
    Ok(CollectCommitsResult {
        inserted_count: upsert_result.inserted,
        skipped_duplicate_count: upsert_result.skipped,
        missing_stats_count,
    })
}
