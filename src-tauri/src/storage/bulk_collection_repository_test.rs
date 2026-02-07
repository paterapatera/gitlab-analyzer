//! 一括収集リポジトリのテスト

#[cfg(test)]
mod tests {
    use crate::storage::bulk_collection_repository::{
        get_collection_targets_with_connection, get_status_with_connection,
        get_targets_by_status_with_connection, record_target_result_with_connection,
        register_targets_with_connection, start_run_with_connection,
    };
    use rusqlite::Connection;

    fn create_test_connection() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("sqlite/migrations/001_init.sql"))
            .unwrap();
        conn.execute_batch(include_str!("sqlite/migrations/006_bulk_collection.sql"))
            .unwrap();
        conn
    }

    fn seed_projects(conn: &Connection) {
        conn.execute(
            "INSERT INTO projects (project_id, name, path_with_namespace, web_url, last_sync_time_utc)
             VALUES (1, 'project-a', 'group/project-a', 'https://gitlab.example.com/group/project-a', '2026-02-01T00:00:00Z')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO projects (project_id, name, path_with_namespace, web_url, last_sync_time_utc)
             VALUES (2, 'project-b', 'group/project-b', 'https://gitlab.example.com/group/project-b', '2026-02-01T00:00:00Z')",
            [],
        )
        .unwrap();
    }

    fn seed_commits(conn: &Connection) {
        seed_projects(conn);
        conn.execute(
            "INSERT INTO commits (project_id, branch_name, sha, author_name, author_email, committed_date_utc, additions, deletions)
             VALUES (1, 'main', 'a1', 'user', 'user@example.com', '2026-02-01T00:00:00Z', 1, 1)",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO commits (project_id, branch_name, sha, author_name, author_email, committed_date_utc, additions, deletions)
             VALUES (2, 'dev', 'b1', 'user', 'user@example.com', '2026-02-01T00:00:00Z', 2, 1)",
            [],
        )
        .unwrap();
    }

    #[test]
    fn test_start_and_register_targets() {
        let mut conn = create_test_connection();
        let run_id = start_run_with_connection(&conn, 2).unwrap();

        let targets = vec![(1, "main".to_string()), (2, "dev".to_string())];
        register_targets_with_connection(&mut conn, &run_id, &targets).unwrap();

        let pending = get_targets_by_status_with_connection(&conn, &run_id, "pending").unwrap();
        assert_eq!(pending.len(), 2);
    }

    #[test]
    fn test_record_results_and_status_summary() {
        let mut conn = create_test_connection();
        let run_id = start_run_with_connection(&conn, 2).unwrap();
        let targets = vec![(1, "main".to_string()), (2, "dev".to_string())];
        register_targets_with_connection(&mut conn, &run_id, &targets).unwrap();

        record_target_result_with_connection(&conn, &run_id, 1, "main", true, Some(3), None)
            .unwrap();
        record_target_result_with_connection(&conn, &run_id, 2, "dev", false, None, Some("failed"))
            .unwrap();

        let status = get_status_with_connection(&conn, &run_id, true).unwrap();
        assert_eq!(status.total_targets, 2);
        assert_eq!(status.completed_count, 2);
        assert_eq!(status.success_count, 1);
        assert_eq!(status.failed_count, 1);
        assert_eq!(status.results.len(), 2);
    }

    #[test]
    fn test_failed_targets_query() {
        let mut conn = create_test_connection();
        let run_id = start_run_with_connection(&conn, 2).unwrap();
        let targets = vec![(1, "main".to_string()), (2, "dev".to_string())];
        register_targets_with_connection(&mut conn, &run_id, &targets).unwrap();

        record_target_result_with_connection(&conn, &run_id, 1, "main", true, Some(1), None)
            .unwrap();
        record_target_result_with_connection(&conn, &run_id, 2, "dev", false, None, Some("error"))
            .unwrap();

        let failed = get_targets_by_status_with_connection(&conn, &run_id, "failed").unwrap();
        assert_eq!(failed, vec![(2, "dev".to_string())]);
    }

    #[test]
    fn test_collection_targets_from_commits() {
        let conn = create_test_connection();
        seed_commits(&conn);

        let targets = get_collection_targets_with_connection(&conn).unwrap();
        assert_eq!(targets.len(), 2);
    }
}
