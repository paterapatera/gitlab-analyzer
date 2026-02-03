//! コミットエンティティ
//!
//! GitLab コミットを表す。stats 欠損時は 0 として扱い、欠損フラグを立てる。

use crate::gitlab::GitLabCommit;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// コミット
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Commit {
    /// プロジェクト ID
    pub project_id: i64,
    
    /// ブランチ名
    pub branch_name: String,
    
    /// コミット SHA
    pub sha: String,
    
    /// コミットメッセージ
    pub message: String,
    
    /// コミット日時（UTC）
    pub committed_date_utc: DateTime<Utc>,
    
    /// 作者名
    pub author_name: String,
    
    /// 作者メールアドレス
    /// NOTE: この値は UI とログに出力しない（FR-017/FR-019）
    pub author_email: Option<String>,
    
    /// 追加行数（stats 欠損時は 0）
    pub additions: i64,
    
    /// 削除行数（stats 欠損時は 0）
    pub deletions: i64,
    
    /// stats が欠損していたかどうか
    pub stats_missing: bool,
}

impl Commit {
    /// 一意キー（プロジェクト ID + ブランチ名 + SHA）
    pub fn unique_key(&self) -> String {
        format!("{}:{}:{}", self.project_id, self.branch_name, self.sha)
    }
    
    /// 合計行数（追加 + 削除）
    pub fn total_lines(&self) -> i64 {
        self.additions + self.deletions
    }
    
    /// ユーザーキー（集計用）
    ///
    /// `author_email` があればそれを優先、なければ `author_name` を使用。
    pub fn user_key(&self) -> String {
        self.author_email.clone().unwrap_or_else(|| self.author_name.clone())
    }
    
    /// 表示名
    pub fn display_name(&self) -> &str {
        &self.author_name
    }
    
    /// コミット月（1-12、UTC 基準）
    pub fn month(&self) -> u32 {
        self.committed_date_utc.month()
    }
    
    /// コミット年（UTC 基準）
    pub fn year(&self) -> i32 {
        self.committed_date_utc.year()
    }
    
    /// GitLab API レスポンスから変換
    pub fn from_gitlab(project_id: i64, branch_name: &str, commit: GitLabCommit) -> Self {
        let (additions, deletions, stats_missing) = match commit.stats {
            Some(stats) => (stats.additions, stats.deletions, false),
            None => (0, 0, true),
        };
        
        // 日時のパース（失敗時は現在時刻）
        let committed_date_utc = DateTime::parse_from_rfc3339(&commit.committed_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        
        Self {
            project_id,
            branch_name: branch_name.to_string(),
            sha: commit.id,
            message: commit.message,
            committed_date_utc,
            author_name: commit.author_name,
            author_email: commit.author_email,
            additions,
            deletions,
            stats_missing,
        }
    }
}

// chrono のトレイトをインポート
use chrono::Datelike;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gitlab::{GitLabCommit, GitLabCommitStats};

    #[test]
    fn test_from_gitlab_with_stats() {
        let gitlab_commit = GitLabCommit {
            id: "abc123".to_string(),
            message: "Initial commit".to_string(),
            committed_date: "2026-01-15T10:00:00Z".to_string(),
            author_name: "John Doe".to_string(),
            author_email: Some("john@example.com".to_string()),
            stats: Some(GitLabCommitStats {
                additions: 100,
                deletions: 50,
                total: 150,
            }),
        };
        
        let commit = Commit::from_gitlab(1, "main", gitlab_commit);
        
        assert_eq!(commit.project_id, 1);
        assert_eq!(commit.branch_name, "main");
        assert_eq!(commit.sha, "abc123");
        assert_eq!(commit.additions, 100);
        assert_eq!(commit.deletions, 50);
        assert!(!commit.stats_missing);
    }

    #[test]
    fn test_from_gitlab_without_stats() {
        let gitlab_commit = GitLabCommit {
            id: "abc123".to_string(),
            message: "Commit without stats".to_string(),
            committed_date: "2026-01-15T10:00:00Z".to_string(),
            author_name: "John Doe".to_string(),
            author_email: None,
            stats: None,
        };
        
        let commit = Commit::from_gitlab(1, "main", gitlab_commit);
        
        // stats 欠損時は 0 として扱う
        assert_eq!(commit.additions, 0);
        assert_eq!(commit.deletions, 0);
        assert!(commit.stats_missing);
    }

    #[test]
    fn test_unique_key() {
        let commit = Commit {
            project_id: 1,
            branch_name: "main".to_string(),
            sha: "abc123".to_string(),
            message: "test".to_string(),
            committed_date_utc: Utc::now(),
            author_name: "John".to_string(),
            author_email: Some("john@example.com".to_string()),
            additions: 10,
            deletions: 5,
            stats_missing: false,
        };
        
        assert_eq!(commit.unique_key(), "1:main:abc123");
    }

    #[test]
    fn test_user_key_prefers_email() {
        let commit = Commit {
            project_id: 1,
            branch_name: "main".to_string(),
            sha: "abc".to_string(),
            message: "test".to_string(),
            committed_date_utc: Utc::now(),
            author_name: "John Doe".to_string(),
            author_email: Some("john@example.com".to_string()),
            additions: 0,
            deletions: 0,
            stats_missing: false,
        };
        
        assert_eq!(commit.user_key(), "john@example.com");
    }

    #[test]
    fn test_user_key_fallback_to_name() {
        let commit = Commit {
            project_id: 1,
            branch_name: "main".to_string(),
            sha: "abc".to_string(),
            message: "test".to_string(),
            committed_date_utc: Utc::now(),
            author_name: "John Doe".to_string(),
            author_email: None,
            additions: 0,
            deletions: 0,
            stats_missing: false,
        };
        
        assert_eq!(commit.user_key(), "John Doe");
    }
    
    #[test]
    fn test_month_utc_based() {
        let commit = Commit {
            project_id: 1,
            branch_name: "main".to_string(),
            sha: "abc".to_string(),
            message: "test".to_string(),
            committed_date_utc: DateTime::parse_from_rfc3339("2026-06-15T23:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
            author_name: "John".to_string(),
            author_email: None,
            additions: 0,
            deletions: 0,
            stats_missing: false,
        };
        
        assert_eq!(commit.month(), 6);
        assert_eq!(commit.year(), 2026);
    }
}
