//! プロジェクトリポジトリ
//!
//! Project の永続化を担当する。

use crate::domain::Project;
use crate::error::AppResult;
use crate::storage::{read_json, write_json, AppData, DATA_FILE_NAME};

/// プロジェクトリポジトリ
pub struct ProjectRepository;

impl ProjectRepository {
    /// 全プロジェクトを取得
    pub fn find_all() -> AppResult<Vec<Project>> {
        let data = read_json::<AppData>(DATA_FILE_NAME)?;
        Ok(data.map(|d| d.projects).unwrap_or_default())
    }
    
    /// プロジェクト一覧を置換（同期時に使用）
    pub fn replace_all(projects: Vec<Project>) -> AppResult<()> {
        let mut data = read_json::<AppData>(DATA_FILE_NAME)?
            .unwrap_or_default();
        
        data.projects = projects;
        
        write_json(DATA_FILE_NAME, &data)
    }
    
    /// プロジェクト ID で検索
    pub fn find_by_id(project_id: i64) -> AppResult<Option<Project>> {
        let projects = Self::find_all()?;
        Ok(projects.into_iter().find(|p| p.project_id == project_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::get_data_file_path;
    use std::fs;
    use serial_test::serial;

    fn cleanup_test_data() {
        if let Ok(path) = get_data_file_path(DATA_FILE_NAME) {
            let _ = fs::remove_file(path);
        }
    }

    #[test]
    #[serial]
    fn test_find_all_empty() {
        cleanup_test_data();
        
        let result = ProjectRepository::find_all();
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    #[serial]
    fn test_replace_all_and_find_all() {
        cleanup_test_data();
        
        let projects = vec![
            Project {
                project_id: 1,
                name: "Project 1".to_string(),
                path_with_namespace: "group/project1".to_string(),
                web_url: "https://gitlab.example.com/group/project1".to_string(),
            },
            Project {
                project_id: 2,
                name: "Project 2".to_string(),
                path_with_namespace: "group/project2".to_string(),
                web_url: "https://gitlab.example.com/group/project2".to_string(),
            },
        ];
        
        let save_result = ProjectRepository::replace_all(projects.clone());
        assert!(save_result.is_ok());
        
        let found = ProjectRepository::find_all().unwrap();
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].project_id, 1);
        
        cleanup_test_data();
    }
}
