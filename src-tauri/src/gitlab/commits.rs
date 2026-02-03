//! GitLab コミット API
//!
//! コミット一覧の取得（ページング、期間指定、stats 付き）。

use crate::error::AppResult;
use crate::gitlab::{GitLabClient, GitLabCommit};

impl GitLabClient {
    /// プロジェクト/ブランチのコミット一覧を取得
    ///
    /// # Arguments
    /// * `project_id` - GitLab プロジェクト ID
    /// * `branch_name` - ブランチ名
    /// * `since` - 開始日時（ISO8601、省略可能）
    /// * `until` - 終了日時（ISO8601、省略可能）
    pub async fn list_commits(
        &self,
        project_id: i64,
        branch_name: &str,
        since: Option<&str>,
        until: Option<&str>,
    ) -> AppResult<Vec<GitLabCommit>> {
        let mut path = format!(
            "/projects/{}/repository/commits?ref_name={}&with_stats=true",
            project_id,
            urlencoding::encode(branch_name)
        );
        
        if let Some(since) = since {
            path.push_str(&format!("&since={}", urlencoding::encode(since)));
        }
        
        if let Some(until) = until {
            path.push_str(&format!("&until={}", urlencoding::encode(until)));
        }
        
        self.get_all_pages(&path).await
    }
    
    /// アクセス可能なプロジェクト一覧を取得
    pub async fn list_projects(&self) -> AppResult<Vec<crate::gitlab::GitLabProject>> {
        let path = "/projects?membership=true&simple=true";
        self.get_all_pages(path).await
    }
}
