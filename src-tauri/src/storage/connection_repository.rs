//! 接続設定リポジトリ
//!
//! GitLabConnection の永続化を担当する。

use crate::domain::GitLabConnection;
use crate::error::AppResult;
use crate::storage::{read_json, write_json, AppData, DATA_FILE_NAME};

/// 接続設定リポジトリ
pub struct ConnectionRepository;

impl ConnectionRepository {
    /// 接続設定を取得
    pub fn get() -> AppResult<Option<GitLabConnection>> {
        let data = read_json::<AppData>(DATA_FILE_NAME)?;
        Ok(data.and_then(|d| d.connection))
    }
    
    /// 接続設定を保存
    pub fn save(connection: GitLabConnection) -> AppResult<()> {
        let mut data = read_json::<AppData>(DATA_FILE_NAME)?
            .unwrap_or_default();
        
        data.connection = Some(connection);
        
        write_json(DATA_FILE_NAME, &data)
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
    fn test_get_nonexistent() {
        cleanup_test_data();
        
        let result = ConnectionRepository::get();
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    #[serial]
    fn test_save_and_get() {
        cleanup_test_data();
        
        let conn = GitLabConnection::new("https://gitlab.example.com", "test-token").unwrap();
        
        let save_result = ConnectionRepository::save(conn.clone());
        assert!(save_result.is_ok());
        
        let get_result = ConnectionRepository::get();
        assert!(get_result.is_ok());
        
        let retrieved = get_result.unwrap();
        assert!(retrieved.is_some());
        
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.base_url, "https://gitlab.example.com");
        assert_eq!(retrieved.access_token, "test-token");
        
        cleanup_test_data();
    }
}
