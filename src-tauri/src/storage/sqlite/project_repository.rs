/// GitLab プロジェクト情報の SQLite リポジトリ
///
/// プロジェクト情報（project_id、name、path、web_url、last_sync_time）を保存・取得します。
use anyhow::{Context, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub project_id: i32,
    pub name: String,
    pub path_with_namespace: String,
    pub web_url: String,
    pub last_sync_time_utc: Option<String>,
}

pub struct ProjectRepository;

impl ProjectRepository {
    /// プロジェクトをデータベースに保存（一括書き込み）
    pub fn save_projects(conn: &mut Connection, projects: Vec<Project>) -> Result<()> {
        let tx = conn.transaction().context("Failed to start transaction")?;

        for project in projects {
            // 既存プロジェクトを削除
            tx.execute(
                "DELETE FROM projects WHERE project_id = ?",
                rusqlite::params![project.project_id],
            )
            .context("Failed to delete existing project")?;

            // 新しいプロジェクトを挿入
            tx.execute(
                "INSERT INTO projects (project_id, name, path_with_namespace, web_url, last_sync_time_utc)
                 VALUES (?, ?, ?, ?, ?)",
                rusqlite::params![
                    project.project_id,
                    project.name,
                    project.path_with_namespace,
                    project.web_url,
                    project.last_sync_time_utc,
                ],
            ).context("Failed to insert project")?;
        }

        tx.commit()
            .context("Failed to commit project transaction")?;

        tracing::info!("Projects saved to database");
        Ok(())
    }

    /// プロジェクト一覧を取得
    pub fn list_projects(conn: &Connection) -> Result<Vec<Project>> {
        let mut stmt = conn
            .prepare(
                "SELECT project_id, name, path_with_namespace, web_url, last_sync_time_utc
             FROM projects ORDER BY name",
            )
            .context("Failed to prepare projects query")?;

        let projects = stmt
            .query_map([], |row| {
                Ok(Project {
                    project_id: row.get(0)?,
                    name: row.get(1)?,
                    path_with_namespace: row.get(2)?,
                    web_url: row.get(3)?,
                    last_sync_time_utc: row.get(4)?,
                })
            })
            .context("Failed to query projects")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect projects")?;

        Ok(projects)
    }

    /// 単一プロジェクトを取得
    pub fn get_project(conn: &Connection, project_id: i32) -> Result<Option<Project>> {
        let mut stmt = conn
            .prepare(
                "SELECT project_id, name, path_with_namespace, web_url, last_sync_time_utc
             FROM projects WHERE project_id = ?",
            )
            .context("Failed to prepare project query")?;

        let result = stmt.query_row(rusqlite::params![project_id], |row| {
            Ok(Project {
                project_id: row.get(0)?,
                name: row.get(1)?,
                path_with_namespace: row.get(2)?,
                web_url: row.get(3)?,
                last_sync_time_utc: row.get(4)?,
            })
        });

        match result {
            Ok(project) => {
                tracing::debug!("Retrieved project: {}", project_id);
                Ok(Some(project))
            }
            Err(rusqlite::Error::QueryReturnedNoRows) => {
                tracing::debug!("Project not found: {}", project_id);
                Ok(None)
            }
            Err(e) => Err(anyhow::anyhow!("Failed to query project: {}", e)),
        }
    }

    /// プロジェクトの同期時刻を更新
    pub fn update_last_sync_time(
        conn: &Connection,
        project_id: i32,
        sync_time: String,
    ) -> Result<()> {
        conn.execute(
            "UPDATE projects SET last_sync_time_utc = ? WHERE project_id = ?",
            rusqlite::params![sync_time, project_id],
        )
        .context("Failed to update last_sync_time")?;

        tracing::debug!("Updated sync time for project: {}", project_id);
        Ok(())
    }

    /// プロジェクトを削除
    pub fn delete_project(conn: &Connection, project_id: i32) -> Result<()> {
        conn.execute(
            "DELETE FROM projects WHERE project_id = ?",
            rusqlite::params![project_id],
        )
        .context("Failed to delete project")?;

        tracing::info!("Project deleted: {}", project_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_connection() -> rusqlite::Connection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();

        // テーブル作成
        conn.execute(
            "CREATE TABLE projects (
                project_id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                path_with_namespace TEXT NOT NULL,
                web_url TEXT NOT NULL,
                last_sync_time_utc TEXT
            )",
            [],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_save_projects() {
        let mut conn = create_test_connection();

        let projects = vec![Project {
            project_id: 1,
            name: "test-project-1".to_string(),
            path_with_namespace: "group/test-project-1".to_string(),
            web_url: "https://gitlab.com/group/test-project-1".to_string(),
            last_sync_time_utc: None,
        }];

        let result = ProjectRepository::save_projects(&mut conn, projects);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_projects() {
        let mut conn = create_test_connection();

        let projects = vec![Project {
            project_id: 1,
            name: "project-1".to_string(),
            path_with_namespace: "group/project-1".to_string(),
            web_url: "https://gitlab.com/group/project-1".to_string(),
            last_sync_time_utc: None,
        }];

        ProjectRepository::save_projects(&mut conn, projects).unwrap();

        let result = ProjectRepository::list_projects(&conn).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].project_id, 1);
    }
}
