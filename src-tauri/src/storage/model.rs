//! ストレージモデル（ルート型）
//!
//! JSON ファイルに保存するデータのルート構造を定義する。

use crate::domain::{GitLabConnection, Project, Commit};
use crate::storage::schema::SchemaVersion;
use serde::{Deserialize, Serialize};

/// データファイル名
pub const DATA_FILE_NAME: &str = "data.json";

/// アプリデータのルート構造
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppData {
    /// スキーマバージョン
    pub schema: SchemaVersion,
    
    /// GitLab 接続設定（1件のみ）
    #[serde(default)]
    pub connection: Option<GitLabConnection>,
    
    /// プロジェクト一覧
    #[serde(default)]
    pub projects: Vec<Project>,
    
    /// コミット一覧
    #[serde(default)]
    pub commits: Vec<Commit>,
}

impl Default for AppData {
    fn default() -> Self {
        Self {
            schema: SchemaVersion::default(),
            connection: None,
            projects: Vec::new(),
            commits: Vec::new(),
        }
    }
}

impl AppData {
    /// 新規作成
    pub fn new() -> Self {
        Self::default()
    }
}
