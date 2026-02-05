/// ユーザーフィルタリポジトリのテスト

#[cfg(test)]
mod tests {
    use crate::storage::sqlite::{run_migrations, UserFilterRepository};
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
    fn test_set_and_get_user_filter() {
        let conn = create_test_connection();

        let users = vec![
            "user1@example.com".to_string(),
            "user2@example.com".to_string(),
        ];

        let set_result = UserFilterRepository::set_user_filter(
            &conn,
            "project-view",
            "project:1",
            users.clone(),
        );

        assert!(set_result.is_ok());

        let retrieved =
            UserFilterRepository::get_user_filter(&conn, "project-view", "project:1").unwrap();

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), users);
    }

    #[test]
    fn test_get_nonexistent_filter() {
        let conn = create_test_connection();

        let retrieved =
            UserFilterRepository::get_user_filter(&conn, "project-view", "project:999").unwrap();

        assert!(retrieved.is_none());
    }

    #[test]
    fn test_update_filter_overwrites() {
        let conn = create_test_connection();

        let users1 = vec!["user1@example.com".to_string()];
        let users2 = vec!["user2@example.com".to_string(), "user3@example.com".to_string()];

        UserFilterRepository::set_user_filter(&conn, "project-view", "project:1", users1).unwrap();
        UserFilterRepository::set_user_filter(&conn, "project-view", "project:1", users2.clone())
            .unwrap();

        let retrieved =
            UserFilterRepository::get_user_filter(&conn, "project-view", "project:1")
                .unwrap()
                .unwrap();

        assert_eq!(retrieved, users2);
    }

    #[test]
    fn test_delete_filter() {
        let conn = create_test_connection();

        let users = vec!["user@example.com".to_string()];

        UserFilterRepository::set_user_filter(&conn, "project-view", "project:1", users).unwrap();

        let delete_result =
            UserFilterRepository::delete_user_filter(&conn, "project-view", "project:1");

        assert!(delete_result.is_ok());

        let retrieved =
            UserFilterRepository::get_user_filter(&conn, "project-view", "project:1").unwrap();

        assert!(retrieved.is_none());
    }

    #[test]
    fn test_multiple_view_types() {
        let conn = create_test_connection();

        let users1 = vec!["user1@example.com".to_string()];
        let users2 = vec!["user2@example.com".to_string()];

        UserFilterRepository::set_user_filter(&conn, "project-view", "context1", users1.clone())
            .unwrap();
        UserFilterRepository::set_user_filter(&conn, "cross-view", "context1", users2.clone())
            .unwrap();

        let retrieved1 =
            UserFilterRepository::get_user_filter(&conn, "project-view", "context1")
                .unwrap()
                .unwrap();
        let retrieved2 = UserFilterRepository::get_user_filter(&conn, "cross-view", "context1")
            .unwrap()
            .unwrap();

        assert_eq!(retrieved1, users1);
        assert_eq!(retrieved2, users2);
    }

    #[test]
    fn test_delete_all_filters_by_view() {
        let conn = create_test_connection();

        let users = vec!["user@example.com".to_string()];

        UserFilterRepository::set_user_filter(&conn, "project-view", "context1", users.clone())
            .unwrap();
        UserFilterRepository::set_user_filter(&conn, "project-view", "context2", users.clone())
            .unwrap();
        UserFilterRepository::set_user_filter(&conn, "cross-view", "context1", users).unwrap();

        let delete_result = UserFilterRepository::delete_all_filters_by_view(&conn, "project-view");

        assert!(delete_result.is_ok());

        // project-view のフィルタはすべて削除される
        let retrieved1 =
            UserFilterRepository::get_user_filter(&conn, "project-view", "context1").unwrap();
        let retrieved2 =
            UserFilterRepository::get_user_filter(&conn, "project-view", "context2").unwrap();

        assert!(retrieved1.is_none());
        assert!(retrieved2.is_none());

        // cross-view のフィルタは残される
        let retrieved3 = UserFilterRepository::get_user_filter(&conn, "cross-view", "context1")
            .unwrap();

        assert!(retrieved3.is_some());
    }
}
