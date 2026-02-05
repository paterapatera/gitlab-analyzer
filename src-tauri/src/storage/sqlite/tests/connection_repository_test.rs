/// GitLab 接続設定リポジトリのテスト

#[cfg(test)]
mod tests {
    use crate::storage::sqlite::{
        connection_repository::ConnectionRepository, run_migrations,
    };
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
    fn test_set_and_get_connection() {
        let conn = create_test_connection();

        let result = ConnectionRepository::set_connection(
            &conn,
            "https://gitlab.example.com".to_string(),
            Some("user@example.com".to_string()),
            "test_token_123".to_string(),
        );

        assert!(result.is_ok());

        // 接続設定を取得
        let get_result = ConnectionRepository::get_connection(&conn);
        assert!(get_result.is_ok());

        let retrieved = get_result.unwrap();
        assert!(retrieved.is_some());

        let (base_url, author_email) = retrieved.unwrap();
        assert_eq!(base_url, "https://gitlab.example.com");
        assert_eq!(author_email, Some("user@example.com".to_string()));
    }

    #[test]
    fn test_get_nonexistent_connection() {
        let conn = create_test_connection();

        let result = ConnectionRepository::get_connection(&conn);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_update_connection_overwrites_previous() {
        let conn = create_test_connection();

        // 最初の接続情報を設定
        ConnectionRepository::set_connection(
            &conn,
            "https://gitlab1.example.com".to_string(),
            Some("user1@example.com".to_string()),
            "token1".to_string(),
        )
        .unwrap();

        // 2回目の接続情報を設定（上書き）
        ConnectionRepository::set_connection(
            &conn,
            "https://gitlab2.example.com".to_string(),
            Some("user2@example.com".to_string()),
            "token2".to_string(),
        )
        .unwrap();

        // 最新の接続情報が取得されることを確認
        let retrieved = ConnectionRepository::get_connection(&conn)
            .unwrap()
            .unwrap();
        let (base_url, author_email) = retrieved;

        assert_eq!(base_url, "https://gitlab2.example.com");
        assert_eq!(author_email, Some("user2@example.com".to_string()));
    }

    #[test]
    fn test_delete_connection() {
        let conn = create_test_connection();

        // 接続情報を設定
        ConnectionRepository::set_connection(
            &conn,
            "https://gitlab.example.com".to_string(),
            Some("user@example.com".to_string()),
            "test_token".to_string(),
        )
        .unwrap();

        // 削除
        let delete_result = ConnectionRepository::delete_connection(&conn);
        assert!(delete_result.is_ok());

        // 削除後は None になることを確認
        let retrieved = ConnectionRepository::get_connection(&conn).unwrap();
        assert!(retrieved.is_none());
    }
}
