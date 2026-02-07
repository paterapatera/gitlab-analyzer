//! GitLab 接続設定エンティティ
//!
//! GitLab への接続情報（ベース URL、アクセストークン）を管理する。

use crate::error::{AppError, AppResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// GitLab 接続設定
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabConnection {
    /// GitLab ベース URL（例: https://gitlab.example.com）
    pub base_url: String,

    /// アクセストークン
    /// NOTE: この値は UI とログに出力しない（FR-016/FR-018）
    pub access_token: String,

    /// 最終更新日時（UTC）
    pub updated_at_utc: DateTime<Utc>,
}

impl GitLabConnection {
    /// 新規作成
    ///
    /// # Validation
    /// - `base_url` は `http://` または `https://` で始まる必要がある
    /// - `access_token` は空でない必要がある
    pub fn new(base_url: &str, access_token: &str) -> AppResult<Self> {
        Self::validate_base_url(base_url)?;
        Self::validate_access_token(access_token)?;

        Ok(Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            access_token: access_token.to_string(),
            updated_at_utc: Utc::now(),
        })
    }

    /// ベース URL のバリデーション
    pub fn validate_base_url(url: &str) -> AppResult<()> {
        if url.is_empty() {
            return Err(AppError::Validation(
                "ベース URL を入力してください".to_string(),
            ));
        }

        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(AppError::Validation(
                "ベース URL は http:// または https:// で始まる必要があります".to_string(),
            ));
        }

        Ok(())
    }

    /// アクセストークンのバリデーション
    pub fn validate_access_token(token: &str) -> AppResult<()> {
        if token.is_empty() {
            return Err(AppError::Validation(
                "アクセストークンを入力してください".to_string(),
            ));
        }

        Ok(())
    }
}

/// UI 返却用の接続情報（トークン非公開）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitLabConnectionPublic {
    /// GitLab ベース URL
    #[serde(rename = "baseUrl", alias = "base_url")]
    pub base_url: String,

    /// 最終更新日時（ISO8601）
    #[serde(rename = "updatedAtUtc", alias = "updated_at_utc")]
    pub updated_at_utc: String,
}

impl From<&GitLabConnection> for GitLabConnectionPublic {
    fn from(conn: &GitLabConnection) -> Self {
        Self {
            base_url: conn.base_url.clone(),
            updated_at_utc: conn.updated_at_utc.to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_connection() {
        let conn = GitLabConnection::new("https://gitlab.example.com", "glpat-xxxx");
        assert!(conn.is_ok());

        let conn = conn.unwrap();
        assert_eq!(conn.base_url, "https://gitlab.example.com");
        assert_eq!(conn.access_token, "glpat-xxxx");
    }

    #[test]
    fn test_new_removes_trailing_slash() {
        let conn = GitLabConnection::new("https://gitlab.example.com/", "glpat-xxxx").unwrap();
        assert_eq!(conn.base_url, "https://gitlab.example.com");
    }

    #[test]
    fn test_validate_base_url_empty() {
        let result = GitLabConnection::validate_base_url("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_base_url_no_protocol() {
        let result = GitLabConnection::validate_base_url("gitlab.example.com");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_access_token_empty() {
        let result = GitLabConnection::validate_access_token("");
        assert!(result.is_err());
    }

    #[test]
    fn test_public_conversion_hides_token() {
        let conn = GitLabConnection::new("https://gitlab.example.com", "secret-token").unwrap();
        let public = GitLabConnectionPublic::from(&conn);

        // public にはトークンが含まれない
        let json = serde_json::to_string(&public).unwrap();
        assert!(!json.contains("secret-token"));
        assert!(json.contains("https://gitlab.example.com"));
    }
}
