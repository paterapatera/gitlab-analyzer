//! ブランチエンティティ
//!
//! GitLab ブランチを表す。

use crate::gitlab::GitLabBranch;
use serde::{Deserialize, Serialize};

/// ブランチ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Branch {
    /// プロジェクト ID
    #[serde(rename = "projectId", alias = "project_id")]
    pub project_id: i64,
    
    /// ブランチ名
    pub name: String,
    
    /// デフォルトブランチかどうか
    #[serde(default, rename = "isDefault", alias = "is_default")]
    pub is_default: bool,
}

impl Branch {
    /// GitLab API レスポンスから変換
    pub fn from_gitlab(project_id: i64, branch: GitLabBranch) -> Self {
        Self {
            project_id,
            name: branch.name,
            is_default: branch.default,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_gitlab_branch() {
        let gitlab_branch = GitLabBranch {
            name: "main".to_string(),
            default: true,
        };
        
        let branch = Branch::from_gitlab(123, gitlab_branch);
        
        assert_eq!(branch.project_id, 123);
        assert_eq!(branch.name, "main");
        assert!(branch.is_default);
    }
}
