//! ストレージモデル（ルート型）
//!
//! JSON ファイルに保存するデータのルート構造を定義する。

use crate::domain::{Commit, GitLabConnection, Project};
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

/// 一括収集の実行状態
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkCollectionStatus {
    pub run_id: String,
    pub status: String,
    pub total_targets: i64,
    pub completed_count: i64,
    pub success_count: i64,
    pub failed_count: i64,
    pub started_at: String,
    pub completed_at: Option<String>,
    #[serde(default)]
    pub results: Vec<BulkCollectionTargetResult>,
}

/// 一括収集の対象ごとの結果
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkCollectionTargetResult {
    pub project_id: i64,
    pub branch_name: String,
    pub status: String,
    pub new_commits_count: Option<i64>,
    pub error_message: Option<String>,
    pub processed_at: Option<String>,
}
