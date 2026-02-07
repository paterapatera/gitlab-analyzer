//! コマンド: ブランチ一覧取得
//!
//! 指定プロジェクトのブランチ一覧を取得する。

use crate::domain::Branch;
use crate::error::{AppError, AppResult};
use crate::gitlab::GitLabClient;
use crate::storage::ConnectionRepository;
use tracing::info;

/// ブランチ一覧を取得
#[tauri::command]
#[allow(non_snake_case)]
pub async fn list_branches(projectId: i64) -> Result<Vec<Branch>, String> {
    list_branches_inner(projectId)
        .await
        .map_err(|e| e.user_message())
}

async fn list_branches_inner(project_id: i64) -> AppResult<Vec<Branch>> {
    // 接続設定を取得
    let connection = ConnectionRepository::get()?.ok_or(AppError::ConnectionNotConfigured)?;

    info!("ブランチ一覧取得: project_id={}", project_id);

    // GitLab API からブランチ一覧を取得
    let client = GitLabClient::new(&connection.base_url, &connection.access_token)?;
    let gitlab_branches = client.list_branches(project_id).await?;

    // ドメインモデルに変換
    let branches: Vec<Branch> = gitlab_branches
        .into_iter()
        .map(|b| Branch::from_gitlab(project_id, b))
        .collect();

    info!("取得したブランチ数: {}", branches.len());

    Ok(branches)
}
