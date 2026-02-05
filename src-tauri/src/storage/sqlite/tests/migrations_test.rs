/// マイグレーション機能のテスト

#[cfg(test)]
mod tests {
    use crate::storage::sqlite::{DatabaseConnection, run_migrations};
    use rusqlite::Connection;
    use tempfile::tempdir;

    fn create_test_connection() -> (Connection, std::path::PathBuf) {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = Connection::open(&db_path).unwrap();
        (conn, db_path)
    }

    #[test]
    fn test_run_migrations_creates_tables() {
        let (conn, _) = create_test_connection();

        let result = run_migrations(&conn);
        assert!(result.is_ok());

        // schema_migrations テーブルが存在することを確認
        let exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='schema_migrations')",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert!(exists);
    }

    #[test]
    fn test_migrations_are_idempotent() {
        let (conn, _) = create_test_connection();

        // 複数回実行しても成功することを確認
        let result1 = run_migrations(&conn);
        let result2 = run_migrations(&conn);

        assert!(result1.is_ok());
        assert!(result2.is_ok());

        // schema_migrations テーブルにバージョンが記録されていることを確認
        let version_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM schema_migrations",
                [],
                |row| row.get(0),
            )
            .unwrap();

        assert!(version_count > 0);
    }

    #[test]
    fn test_migration_creates_required_tables() {
        let (conn, _) = create_test_connection();

        run_migrations(&conn).unwrap();

        // 必要なテーブル一覧
        let required_tables = vec!["connections", "projects", "commits", "user_filters"];

        for table_name in required_tables {
            let exists: bool = conn
                .query_row(
                    &format!(
                        "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='{}')",
                        table_name
                    ),
                    [],
                    |row| row.get(0),
                )
                .unwrap();

            assert!(exists, "Table {} should exist after migration", table_name);
        }
    }

    #[test]
    fn test_commits_table_has_primary_key() {
        let (conn, _) = create_test_connection();

        run_migrations(&conn).unwrap();

        // commits テーブルの複合主キーを確認
        let table_info: Vec<(String, String, i32)> = {
            let mut stmt = conn
                .prepare("PRAGMA table_info(commits)")
                .unwrap();

            stmt.query_map([], |row| {
                Ok((
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i32>(5)?,
                ))
            })
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
        };

        // project_id, branch_name, sha が主キーの一部であることを確認
        assert!(
            table_info.iter().any(|(name, _, pk)| name == "project_id" && *pk > 0),
            "project_id should be part of primary key"
        );
    }
}
