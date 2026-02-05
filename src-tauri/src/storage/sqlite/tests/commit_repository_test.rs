/// コミットリポジトリのテスト

#[cfg(test)]
mod tests {
    use crate::storage::sqlite::{commit_repository::Commit, run_migrations, CommitRepository};
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
    fn test_save_commits() {
        let _dir = tempdir().unwrap();
        let db_path = _dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        run_migrations(&conn).unwrap();

        let commits = vec![
            Commit {
                project_id: 1,
                branch_name: "main".to_string(),
                sha: "abc123".to_string(),
                author_name: "Author One".to_string(),
                author_email: "author1@example.com".to_string(),
                committed_date_utc: "2024-01-01T10:00:00Z".to_string(),
                additions: 50,
                deletions: 20,
            },
            Commit {
                project_id: 1,
                branch_name: "main".to_string(),
                sha: "def456".to_string(),
                author_name: "Author Two".to_string(),
                author_email: "author2@example.com".to_string(),
                committed_date_utc: "2024-01-02T14:30:00Z".to_string(),
                additions: 100,
                deletions: 50,
            },
        ];

        let result = CommitRepository::save_commits(&mut conn, commits);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 2);
    }

    #[test]
    fn test_get_commits_by_project() {
        let _dir = tempdir().unwrap();
        let db_path = _dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        run_migrations(&conn).unwrap();

        let commits = vec![
            Commit {
                project_id: 1,
                branch_name: "main".to_string(),
                sha: "abc123".to_string(),
                author_name: "Test".to_string(),
                author_email: "test@example.com".to_string(),
                committed_date_utc: "2024-01-01T00:00:00Z".to_string(),
                additions: 10,
                deletions: 5,
            },
        ];

        CommitRepository::save_commits(&mut conn, commits).unwrap();

        let retrieved = CommitRepository::get_commits_by_project(&conn, 1).unwrap();
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].sha, "abc123");
    }

    #[test]
    fn test_get_commits_by_branch() {
        let _dir = tempdir().unwrap();
        let db_path = _dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        run_migrations(&conn).unwrap();

        let commits = vec![
            Commit {
                project_id: 1,
                branch_name: "main".to_string(),
                sha: "abc123".to_string(),
                author_name: "Test".to_string(),
                author_email: "test@example.com".to_string(),
                committed_date_utc: "2024-01-01T00:00:00Z".to_string(),
                additions: 10,
                deletions: 5,
            },
            Commit {
                project_id: 1,
                branch_name: "develop".to_string(),
                sha: "def456".to_string(),
                author_name: "Test".to_string(),
                author_email: "test@example.com".to_string(),
                committed_date_utc: "2024-01-01T00:00:00Z".to_string(),
                additions: 20,
                deletions: 10,
            },
        ];

        CommitRepository::save_commits(&mut conn, commits).unwrap();

        let retrieved = CommitRepository::get_commits_by_branch(&conn, 1, "main").unwrap();
        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].branch_name, "main");
    }

    #[test]
    fn test_get_commits_by_date_range() {
        let _dir = tempdir().unwrap();
        let db_path = _dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        run_migrations(&conn).unwrap();

        let commits = vec![
            Commit {
                project_id: 1,
                branch_name: "main".to_string(),
                sha: "abc123".to_string(),
                author_name: "Test".to_string(),
                author_email: "test@example.com".to_string(),
                committed_date_utc: "2024-01-05T00:00:00Z".to_string(),
                additions: 10,
                deletions: 5,
            },
            Commit {
                project_id: 1,
                branch_name: "main".to_string(),
                sha: "def456".to_string(),
                author_name: "Test".to_string(),
                author_email: "test@example.com".to_string(),
                committed_date_utc: "2024-02-05T00:00:00Z".to_string(),
                additions: 20,
                deletions: 10,
            },
        ];

        CommitRepository::save_commits(&mut conn, commits).unwrap();

        let retrieved = CommitRepository::get_commits_by_date_range(
            &conn,
            1,
            "2024-01-01T00:00:00Z",
            "2024-01-31T23:59:59Z",
        )
        .unwrap();

        assert_eq!(retrieved.len(), 1);
        assert_eq!(retrieved[0].committed_date_utc, "2024-01-05T00:00:00Z");
    }

    #[test]
    fn test_count_commits() {
        let _dir = tempdir().unwrap();
        let db_path = _dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        run_migrations(&conn).unwrap();

        let commits = vec![
            Commit {
                project_id: 1,
                branch_name: "main".to_string(),
                sha: "abc123".to_string(),
                author_name: "Test".to_string(),
                author_email: "test@example.com".to_string(),
                committed_date_utc: "2024-01-01T00:00:00Z".to_string(),
                additions: 10,
                deletions: 5,
            },
            Commit {
                project_id: 1,
                branch_name: "main".to_string(),
                sha: "def456".to_string(),
                author_name: "Test".to_string(),
                author_email: "test@example.com".to_string(),
                committed_date_utc: "2024-01-02T00:00:00Z".to_string(),
                additions: 20,
                deletions: 10,
            },
        ];

        CommitRepository::save_commits(&mut conn, commits).unwrap();

        let count = CommitRepository::count_commits_by_project(&conn, 1).unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_duplicate_commit_prevention() {
        let _dir = tempdir().unwrap();
        let db_path = _dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        run_migrations(&conn).unwrap();

        let commit = Commit {
            project_id: 1,
            branch_name: "main".to_string(),
            sha: "abc123".to_string(),
            author_name: "Test".to_string(),
            author_email: "test@example.com".to_string(),
            committed_date_utc: "2024-01-01T00:00:00Z".to_string(),
            additions: 10,
            deletions: 5,
        };

        // 同じコミットを2回保存
        let result1 = CommitRepository::save_commits(&mut conn, vec![commit.clone()]);
        let result2 = CommitRepository::save_commits(&mut conn, vec![commit]);

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        // コミット数が2でなく1であることを確認（重複排除）
        let count = CommitRepository::count_commits_by_project(&conn, 1).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_get_monthly_stats_by_author() {
        let _dir = tempdir().unwrap();
        let db_path = _dir.path().join("test.db");
        let mut conn = Connection::open(&db_path).unwrap();
        run_migrations(&conn).unwrap();

        let commits = vec![
            Commit {
                project_id: 1,
                branch_name: "main".to_string(),
                sha: "abc123".to_string(),
                author_name: "Test".to_string(),
                author_email: "test@example.com".to_string(),
                committed_date_utc: "2024-01-05T00:00:00Z".to_string(),
                additions: 50,
                deletions: 10,
            },
            Commit {
                project_id: 1,
                branch_name: "main".to_string(),
                sha: "def456".to_string(),
                author_name: "Test".to_string(),
                author_email: "test@example.com".to_string(),
                committed_date_utc: "2024-02-05T00:00:00Z".to_string(),
                additions: 100,
                deletions: 20,
            },
        ];

        CommitRepository::save_commits(&mut conn, commits).unwrap();

        let stats = CommitRepository::get_monthly_stats_by_author(&conn, 1, "test@example.com")
            .unwrap();

        assert_eq!(stats.len(), 2);
        // 直近の月が最初に返される
        assert_eq!(stats[0].0, "2024-02");
        assert_eq!(stats[0].1, 1); // コミット数
        assert_eq!(stats[0].2, 100); // additions
    }
}
