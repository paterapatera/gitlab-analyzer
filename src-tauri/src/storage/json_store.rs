//! JSON ストア
//!
//! JSON ファイルベースの永続化を提供する。
//! アトミックな書き込み（一時ファイル経由）をサポート。

use crate::error::{AppError, AppResult};
use crate::paths::get_data_file_path;
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::PathBuf;

/// JSON ファイルを読み込む
///
/// ファイルが存在しない場合は `None` を返す。
pub fn read_json<T: DeserializeOwned>(filename: &str) -> AppResult<Option<T>> {
    let path = get_data_file_path(filename)?;

    if !path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&path)
        .map_err(|e| AppError::Storage(format!("ファイル読み込み失敗: {}", e)))?;

    let data = serde_json::from_str(&content)
        .map_err(|e| AppError::Storage(format!("JSON パース失敗: {}", e)))?;

    Ok(Some(data))
}

/// JSON ファイルに書き込む（アトミック）
///
/// 一時ファイルに書き込み後、リネームすることでアトミック性を確保。
pub fn write_json<T: Serialize>(filename: &str, data: &T) -> AppResult<()> {
    let path = get_data_file_path(filename)?;
    let temp_path = get_temp_path(&path);

    // JSON シリアライズ
    let content = serde_json::to_string_pretty(data)
        .map_err(|e| AppError::Storage(format!("JSON シリアライズ失敗: {}", e)))?;

    // 一時ファイルに書き込み
    fs::write(&temp_path, &content)
        .map_err(|e| AppError::Storage(format!("一時ファイル書き込み失敗: {}", e)))?;

    // アトミックリネーム
    fs::rename(&temp_path, &path).map_err(|e| {
        // リネーム失敗時は一時ファイルを削除
        let _ = fs::remove_file(&temp_path);
        AppError::Storage(format!("ファイルリネーム失敗: {}", e))
    })?;

    Ok(())
}

/// 一時ファイルパスを生成
fn get_temp_path(path: &PathBuf) -> PathBuf {
    let mut temp = path.clone();
    let filename = temp.file_name().and_then(|n| n.to_str()).unwrap_or("data");
    temp.set_file_name(format!(".{}.tmp", filename));
    temp
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_json::<TestData>("nonexistent_test_file.json");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_write_and_read_json() {
        let test_file = "test_write_read.json";
        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        // 書き込み
        let write_result = write_json(test_file, &data);
        assert!(write_result.is_ok(), "Write failed: {:?}", write_result);

        // 読み込み
        let read_result = read_json::<TestData>(test_file);
        assert!(read_result.is_ok(), "Read failed: {:?}", read_result);

        let read_data = read_result.unwrap();
        assert!(read_data.is_some());
        assert_eq!(read_data.unwrap(), data);

        // クリーンアップ
        if let Ok(path) = get_data_file_path(test_file) {
            let _ = fs::remove_file(path);
        }
    }

    #[test]
    fn test_get_temp_path() {
        let path = PathBuf::from("/some/path/data.json");
        let temp = get_temp_path(&path);
        assert!(temp.to_string_lossy().contains(".data.json.tmp"));
    }
}
