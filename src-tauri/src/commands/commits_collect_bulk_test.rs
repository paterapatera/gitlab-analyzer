//! 一括コミット収集コマンドのテスト

#[cfg(test)]
mod tests {
    use crate::commands::commits_collect_bulk::{
        cancel_bulk_collection, is_cancel_requested, prepare_bulk_collection_start_with_connection,
        reset_cancel_flag,
    };
    use crate::storage::bulk_collection_repository::{
        cancel_run_with_connection, record_target_result_with_connection,
        register_targets_with_connection, start_run_with_connection,
    };
    use rusqlite::Connection;

    fn create_test_connection() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(include_str!("../storage/sqlite/migrations/001_init.sql"))
            .unwrap();
        conn.execute_batch(include_str!(
            "../storage/sqlite/migrations/006_bulk_collection.sql"
        ))
        .unwrap();
        conn
    }

    fn seed_commits(conn: &Connection) {
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
    fn test_prepare_bulk_collection_start_new_run() {
        let mut conn = create_test_connection();
        seed_commits(&conn);

        let context = prepare_bulk_collection_start_with_connection(&mut conn).unwrap();
        assert_eq!(context.total_targets, 2);
        assert_eq!(context.completed_count, 0);
        assert_eq!(context.targets.len(), 2);
    }

    #[test]
    fn test_prepare_bulk_collection_start_resume() {
        let mut conn = create_test_connection();
        seed_commits(&conn);

        let run_id = start_run_with_connection(&conn, 2).unwrap();
        let targets = vec![(1, "main".to_string()), (2, "dev".to_string())];
        register_targets_with_connection(&mut conn, &run_id, &targets).unwrap();

        record_target_result_with_connection(&conn, &run_id, 1, "main", true, Some(1), None)
            .unwrap();
        cancel_run_with_connection(&conn, &run_id).unwrap();

        let context = prepare_bulk_collection_start_with_connection(&mut conn).unwrap();
        assert_eq!(context.run_id, run_id);
        assert_eq!(context.total_targets, 2);
        assert_eq!(context.completed_count, 1);
        assert_eq!(context.targets.len(), 1);
    }

    #[test]
    fn test_cancel_flag_set() {
        reset_cancel_flag();
        assert!(!is_cancel_requested());

        cancel_bulk_collection().unwrap();
        assert!(is_cancel_requested());
    }
}
