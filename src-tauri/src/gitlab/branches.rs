//! GitLab ブランチ API
//!
//! ブランチ一覧の取得。

use crate::error::AppResult;
use crate::gitlab::{GitLabClient, GitLabBranch};

impl GitLabClient {
    /// プロジェクトのブランチ一覧を取得
    ///
    /// # Arguments
    /// * `project_id` - GitLab プロジェクト ID
    pub async fn list_branches(&self, project_id: i64) -> AppResult<Vec<GitLabBranch>> {
        let path = format!("/projects/{}/repository/branches", project_id);
        self.get_all_pages(&path).await
    }
}
