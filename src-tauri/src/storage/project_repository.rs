//! プロジェクトリポジトリ（SQLite ベース）
//!
//! Project の永続化を SQLite で行います。

use crate::domain::Project;
use crate::error::{AppError, AppResult};
use crate::storage::sqlite;

/// プロジェクトリポジトリ
pub struct ProjectRepository;

impl ProjectRepository {
    /// 全プロジェクトを取得
    pub fn find_all() -> AppResult<Vec<Project>> {
        let conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        let sqlite_projects = sqlite::ProjectRepository::list_projects(&conn)
            .map_err(|e| AppError::Storage(e.to_string()))?;

        // SQLite の Project 型をドメインの Project 型に変換
        let projects = sqlite_projects
            .into_iter()
            .map(|p| Project {
                project_id: p.project_id as i64,
                name: p.name,
                path_with_namespace: p.path_with_namespace,
                web_url: p.web_url,
            })
            .collect();

        Ok(projects)
    }

    /// プロジェクト一覧を置換（同期時に使用）
    pub fn replace_all(projects: Vec<Project>) -> AppResult<()> {
        let mut conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        // ドメインの Project 型を SQLite の Project 型に変換
        let sqlite_projects = projects
            .into_iter()
            .map(|p| sqlite::project_repository::Project {
                project_id: p.project_id as i32,
                name: p.name,
                path_with_namespace: p.path_with_namespace,
                web_url: p.web_url,
                last_sync_time_utc: Some(chrono::Utc::now().to_rfc3339()),
            })
            .collect();

        sqlite::ProjectRepository::save_projects(&mut conn, sqlite_projects)
            .map_err(|e| AppError::Storage(e.to_string()))?;

        Ok(())
    }

    /// プロジェクト ID で検索
    pub fn find_by_id(project_id: i64) -> AppResult<Option<Project>> {
        let conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        let result = sqlite::ProjectRepository::get_project(&conn, project_id as i32)
            .map_err(|e| AppError::Storage(e.to_string()))?;

        Ok(result.map(|p| Project {
            project_id: p.project_id as i64,
            name: p.name,
            path_with_namespace: p.path_with_namespace,
            web_url: p.web_url,
        }))
    }
}
