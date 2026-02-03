//! コマンド: プロジェクト一覧取得
//!
//! ローカルに保存されたプロジェクト一覧を取得する。

use crate::domain::Project;
use crate::error::AppResult;
use crate::storage::ProjectRepository;

/// 保存済みプロジェクト一覧を取得
#[tauri::command]
pub fn get_projects() -> Result<Vec<Project>, String> {
    get_projects_inner().map_err(|e| e.to_string())
}

fn get_projects_inner() -> AppResult<Vec<Project>> {
    ProjectRepository::find_all()
}
