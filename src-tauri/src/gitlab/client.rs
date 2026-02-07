//! GitLab REST API クライアント
//!
//! GitLab API への HTTP リクエストを行う基盤。

use crate::error::{AppError, AppResult};
use crate::logging::mask_sensitive;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;
use tracing::{debug, info};

/// GitLab API クライアント
#[derive(Debug, Clone)]
pub struct GitLabClient {
    /// HTTP クライアント
    client: Client,
    /// GitLab ベース URL
    base_url: String,
    /// アクセストークン
    access_token: String,
}

impl GitLabClient {
    /// 新規クライアントを作成
    ///
    /// # Arguments
    /// * `base_url` - GitLab のベース URL（例: https://gitlab.example.com）
    /// * `access_token` - GitLab アクセストークン
    pub fn new(base_url: &str, access_token: &str) -> AppResult<Self> {
        // トレイリングスラッシュを除去
        let base_url = base_url.trim_end_matches('/').to_string();

        let client = Client::builder()
            .build()
            .map_err(|e| AppError::Internal(format!("HTTP クライアント初期化失敗: {}", e)))?;

        info!(
            "GitLab クライアント作成: base_url={}, token={}",
            base_url,
            mask_sensitive(access_token)
        );

        Ok(Self {
            client,
            base_url,
            access_token: access_token.to_string(),
        })
    }

    /// 認証ヘッダーを生成
    fn auth_headers(&self) -> AppResult<HeaderMap> {
        let mut headers = HeaderMap::new();
        let token_value = format!("Bearer {}", self.access_token);
        let header_value = HeaderValue::from_str(&token_value)
            .map_err(|_| AppError::Validation("無効なトークン形式です".to_string()))?;
        headers.insert(AUTHORIZATION, header_value);
        Ok(headers)
    }

    /// API エンドポイントの URL を構築
    pub fn api_url(&self, path: &str) -> String {
        format!("{}/api/v4{}", self.base_url, path)
    }

    /// GET リクエストを実行
    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> AppResult<T> {
        let url = self.api_url(path);
        debug!("GitLab API GET: {}", url);

        let headers = self.auth_headers()?;

        let response = self
            .client
            .get(&url)
            .headers(headers)
            .send()
            .await
            .map_err(|e| AppError::GitLabApi {
                message: format!("リクエスト失敗: {}", e),
                guidance: "ネットワーク接続を確認してください。".to_string(),
            })?;

        let status = response.status();

        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::from_gitlab_status(status.as_u16(), &error_text));
        }

        let data = response
            .json::<T>()
            .await
            .map_err(|e| AppError::GitLabApi {
                message: format!("レスポンスパース失敗: {}", e),
                guidance: "GitLab API のレスポンス形式が変更された可能性があります。".to_string(),
            })?;

        Ok(data)
    }

    /// GET リクエストを実行（ページング対応）
    ///
    /// 全ページを取得して結合した結果を返す。
    pub async fn get_all_pages<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> AppResult<Vec<T>> {
        let mut all_items = Vec::new();
        let mut page = 1;
        let per_page = 100;

        loop {
            let separator = if path.contains('?') { "&" } else { "?" };
            let paginated_path =
                format!("{}{}page={}&per_page={}", path, separator, page, per_page);

            let items: Vec<T> = self.get(&paginated_path).await?;

            let count = items.len();
            all_items.extend(items);

            if count < per_page {
                break;
            }

            page += 1;
        }

        Ok(all_items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_url() {
        let client = GitLabClient::new("https://gitlab.example.com", "test-token").unwrap();
        assert_eq!(
            client.api_url("/projects"),
            "https://gitlab.example.com/api/v4/projects"
        );
    }

    #[test]
    fn test_api_url_trailing_slash() {
        let client = GitLabClient::new("https://gitlab.example.com/", "test-token").unwrap();
        assert_eq!(
            client.api_url("/projects"),
            "https://gitlab.example.com/api/v4/projects"
        );
    }
}
