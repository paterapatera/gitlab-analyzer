/// プロジェクトリポジトリのテスト

#[cfg(test)]
mod tests {
    use crate::storage::sqlite::{project_repository::Project, run_migrations, ProjectRepository};
    use rusqlite::Connection;
    use tempfile::tempdir;

    fn create_test_connection() -> Connection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = Connection::open(&db_path).unwrap();

        // マイグレーション実行
        run_migrations(&conn).unwrap();

        conn
    }

    #[test]
    fn test_save_and_list_projects() {
        let _dir = tempdir().unwrap();
        let db_path = _dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();

        run_migrations(&conn).unwrap();

        let projects = vec![
            Project {
                project_id: 1,
                name: "project-1".to_string(),
                path_with_namespace: "group/project-1".to_string(),
                web_url: "https://gitlab.com/group/project-1".to_string(),
                last_sync_time_utc: None,
            },
            Project {
                project_id: 2,
                name: "project-2".to_string(),
                path_with_namespace: "group/project-2".to_string(),
                web_url: "https://gitlab.com/group/project-2".to_string(),
                last_sync_time_utc: None,
            },
        ];

        let save_result = ProjectRepository::save_projects(&mut conn, projects);
        assert!(save_result.is_ok());

        // プロジェクト一覧を取得
        let list_result = ProjectRepository::list_projects(&conn);
        assert!(list_result.is_ok());

        let retrieved = list_result.unwrap();
        assert_eq!(retrieved.len(), 2);
        assert_eq!(retrieved[0].project_id, 1);
        assert_eq!(retrieved[1].project_id, 2);
    }

    #[test]
    fn test_get_single_project() {
        let _dir = tempdir().unwrap();
        let db_path = _dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        run_migrations(&conn).unwrap();

        let projects = vec![Project {
            project_id: 1,
            name: "test".to_string(),
            path_with_namespace: "group/test".to_string(),
            web_url: "https://gitlab.com/group/test".to_string(),
            last_sync_time_utc: None,
        }];

        ProjectRepository::save_projects(&mut conn, projects).unwrap();

        let retrieved = ProjectRepository::get_project(&conn, 1).unwrap();
        assert!(retrieved.is_some());

        let project = retrieved.unwrap();
        assert_eq!(project.project_id, 1);
        assert_eq!(project.name, "test");
    }

    #[test]
    fn test_get_nonexistent_project() {
        let conn = create_test_connection();

        let retrieved = ProjectRepository::get_project(&conn, 999).unwrap();
        assert!(retrieved.is_none());
    }

    #[test]
    fn test_update_last_sync_time() {
        let _dir = tempdir().unwrap();
        let db_path = _dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        run_migrations(&conn).unwrap();

        let projects = vec![Project {
            project_id: 1,
            name: "test".to_string(),
            path_with_namespace: "group/test".to_string(),
            web_url: "https://gitlab.com/group/test".to_string(),
            last_sync_time_utc: None,
        }];

        ProjectRepository::save_projects(&mut conn, projects).unwrap();

        let sync_time = "2024-01-01T12:00:00Z".to_string();
        let update_result = ProjectRepository::update_last_sync_time(&conn, 1, sync_time.clone());
        assert!(update_result.is_ok());

        let retrieved = ProjectRepository::get_project(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(retrieved.last_sync_time_utc, Some(sync_time));
    }

    #[test]
    fn test_delete_project() {
        let _dir = tempdir().unwrap();
        let db_path = _dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        run_migrations(&conn).unwrap();

        let projects = vec![Project {
            project_id: 1,
            name: "test".to_string(),
            path_with_namespace: "group/test".to_string(),
            web_url: "https://gitlab.com/group/test".to_string(),
            last_sync_time_utc: None,
        }];

        ProjectRepository::save_projects(&mut conn, projects).unwrap();

        let delete_result = ProjectRepository::delete_project(&conn, 1);
        assert!(delete_result.is_ok());

        let retrieved = ProjectRepository::get_project(&conn, 1).unwrap();
        assert!(retrieved.is_none());
    }
}
