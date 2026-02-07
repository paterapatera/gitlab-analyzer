//! コマンド: プロジェクト同期
//!
//! GitLab からアクセス可能なプロジェクト一覧を取得し、ローカルに保存する。

use crate::domain::Project;
use crate::error::{AppError, AppResult};
use crate::gitlab::GitLabClient;
use crate::storage::{ConnectionRepository, ProjectRepository};
use tracing::info;

/// プロジェクトを同期
#[tauri::command]
pub async fn sync_projects() -> Result<Vec<Project>, String> {
    sync_projects_inner().await.map_err(|e| e.user_message())
}

async fn sync_projects_inner() -> AppResult<Vec<Project>> {
    // 接続設定を取得
    let connection = ConnectionRepository::get()?.ok_or(AppError::ConnectionNotConfigured)?;

    info!("プロジェクト同期開始: {}", connection.base_url);

    // GitLab API からプロジェクト一覧を取得
    let client = GitLabClient::new(&connection.base_url, &connection.access_token)?;
    let gitlab_projects = client.list_projects().await?;

    // ドメインモデルに変換
    let projects: Vec<Project> = gitlab_projects.into_iter().map(Project::from).collect();

    info!("取得したプロジェクト数: {}", projects.len());

    // ローカルに保存
    ProjectRepository::replace_all(projects.clone())?;

    Ok(projects)
}
