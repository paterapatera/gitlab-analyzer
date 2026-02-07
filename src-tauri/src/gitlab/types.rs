//! GitLab API レスポンス型
//!
//! GitLab REST API のレスポンス JSON に対応する型定義。

use serde::{Deserialize, Serialize};

/// GitLab プロジェクト（API レスポンス）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabProject {
    /// プロジェクト ID
    pub id: i64,

    /// プロジェクト名
    pub name: String,

    /// 名前空間付きパス（例: group/project）
    pub path_with_namespace: String,

    /// Web UI の URL
    pub web_url: String,
}

/// GitLab ブランチ（API レスポンス）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabBranch {
    /// ブランチ名
    pub name: String,

    /// デフォルトブランチかどうか
    #[serde(default)]
    pub default: bool,
}

/// GitLab コミット（API レスポンス）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabCommit {
    /// コミット SHA
    pub id: String,

    /// コミットメッセージ
    #[serde(default)]
    pub message: String,

    /// コミット日時（ISO8601）
    pub committed_date: String,

    /// 作者名
    pub author_name: String,

    /// 作者メールアドレス（取得できない場合あり）
    pub author_email: Option<String>,

    /// コミット統計（with_stats=true の場合のみ）
    pub stats: Option<GitLabCommitStats>,
}

/// GitLab コミット統計
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabCommitStats {
    /// 追加行数
    pub additions: i64,

    /// 削除行数
    pub deletions: i64,

    /// 総変更行数
    pub total: i64,
}
