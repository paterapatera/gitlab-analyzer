//! アプリケーションパスヘルパー
//!
//! アプリデータディレクトリの解決を行う。

use crate::error::{AppError, AppResult};
use std::path::PathBuf;

/// アプリデータディレクトリ名
const APP_DATA_DIR_NAME: &str = "gitlab-analyzer";

/// アプリデータディレクトリのパスを取得
///
/// Windows: `%APPDATA%/gitlab-analyzer`
/// macOS: `~/Library/Application Support/gitlab-analyzer`
/// Linux: `~/.local/share/gitlab-analyzer`
pub fn get_app_data_dir() -> AppResult<PathBuf> {
    let base = dirs::data_dir().ok_or_else(|| {
        AppError::Internal("アプリデータディレクトリを特定できません".to_string())
    })?;

    Ok(base.join(APP_DATA_DIR_NAME))
}

/// アプリデータディレクトリを確保（存在しなければ作成）
pub fn ensure_app_data_dir() -> AppResult<PathBuf> {
    let dir = get_app_data_dir()?;

    if !dir.exists() {
        std::fs::create_dir_all(&dir)
            .map_err(|e| AppError::Storage(format!("ディレクトリ作成失敗: {}", e)))?;
    }

    Ok(dir)
}

/// データファイルのパスを取得
///
/// 指定されたファイル名に対して、アプリデータディレクトリ内のパスを返す。
pub fn get_data_file_path(filename: &str) -> AppResult<PathBuf> {
    let dir = ensure_app_data_dir()?;
    Ok(dir.join(filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_app_data_dir() {
        let result = get_app_data_dir();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.ends_with(APP_DATA_DIR_NAME));
    }
}
