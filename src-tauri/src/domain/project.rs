//! プロジェクトエンティティ
//!
//! GitLab プロジェクトを表す。

use crate::gitlab::GitLabProject;
use serde::{Deserialize, Serialize};

/// プロジェクト
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Project {
    /// GitLab プロジェクト ID
    #[serde(rename = "projectId", alias = "project_id")]
    pub project_id: i64,
    
    /// プロジェクト名
    pub name: String,
    
    /// 名前空間付きパス（例: group/project）
    #[serde(rename = "pathWithNamespace", alias = "path_with_namespace")]
    pub path_with_namespace: String,
    
    /// Web UI の URL
    #[serde(rename = "webUrl", alias = "web_url")]
    pub web_url: String,
}

impl From<GitLabProject> for Project {
    fn from(p: GitLabProject) -> Self {
        Self {
            project_id: p.id,
            name: p.name,
            path_with_namespace: p.path_with_namespace,
            web_url: p.web_url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_gitlab_project() {
        let gitlab_project = GitLabProject {
            id: 123,
            name: "my-project".to_string(),
            path_with_namespace: "group/my-project".to_string(),
            web_url: "https://gitlab.example.com/group/my-project".to_string(),
        };
        
        let project = Project::from(gitlab_project);
        
        assert_eq!(project.project_id, 123);
        assert_eq!(project.name, "my-project");
        assert_eq!(project.path_with_namespace, "group/my-project");
    }
}
