//! アプリケーションエラー型
//!
//! 全コマンドで共通のエラー型を定義する。
//! フロントエンドには「次に何をすべきか」が分かるメッセージを返す。

use thiserror::Error;

/// アプリケーションエラー
#[derive(Debug, Error)]
pub enum AppError {
    /// GitLab API エラー（認証失敗など）
    #[error("GitLab API エラー: {message}")]
    GitLabApi {
        /// エラーメッセージ
        message: String,
        /// ユーザーへのガイド（次に取るべき行動）
        guidance: String,
    },

    /// 入力バリデーションエラー
    #[error("入力エラー: {0}")]
    Validation(String),

    /// ストレージ（ファイル I/O）エラー
    #[error("データ保存エラー: {0}")]
    Storage(String),

    /// 接続設定が未登録
    #[error("GitLab 接続設定が登録されていません。設定画面で登録してください。")]
    ConnectionNotConfigured,

    /// 内部エラー（予期しないエラー）
    #[error("内部エラー: {0}")]
    Internal(String),
}

impl AppError {
    /// GitLab API の HTTP ステータスコードからエラーを生成
    pub fn from_gitlab_status(status: u16, message: &str) -> Self {
        let (msg, guidance) = match status {
            401 => (
                "認証に失敗しました".to_string(),
                "アクセストークンを確認し、再入力してください。トークンの有効期限が切れている可能性があります。".to_string(),
            ),
            403 => (
                "アクセス権限がありません".to_string(),
                "トークンのスコープ（api または read_api）を確認してください。".to_string(),
            ),
            404 => (
                "リソースが見つかりません".to_string(),
                "プロジェクトが存在するか、アクセス権限があるか確認してください。".to_string(),
            ),
            429 => (
                "API レート制限に達しました".to_string(),
                "しばらく待ってから再試行してください。".to_string(),
            ),
            500..=599 => (
                format!("GitLab サーバーエラー ({})", status),
                "GitLab サーバーに問題が発生しています。しばらく待ってから再試行してください。".to_string(),
            ),
            _ => (
                format!("予期しないエラー ({}): {}", status, message),
                "エラーの詳細を確認し、再試行してください。".to_string(),
            ),
        };

        Self::GitLabApi {
            message: msg,
            guidance,
        }
    }

    /// ユーザー向けのメッセージを取得（ガイダンス付き）
    pub fn user_message(&self) -> String {
        match self {
            Self::GitLabApi { message, guidance } => {
                format!("{}\n\n💡 {}", message, guidance)
            }
            _ => self.to_string(),
        }
    }
}

/// Tauri コマンドのエラー型（シリアライズ可能）
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.user_message())
    }
}

/// Result 型エイリアス
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_gitlab_status_401() {
        let err = AppError::from_gitlab_status(401, "Unauthorized");
        match err {
            AppError::GitLabApi { message, guidance } => {
                assert!(message.contains("認証"));
                assert!(guidance.contains("トークン"));
            }
            _ => panic!("Expected GitLabApi error"),
        }
    }

    #[test]
    fn test_user_message_includes_guidance() {
        let err = AppError::GitLabApi {
            message: "テストエラー".to_string(),
            guidance: "再試行してください".to_string(),
        };
        let msg = err.user_message();
        assert!(msg.contains("テストエラー"));
        assert!(msg.contains("再試行"));
        assert!(msg.contains("💡"));
    }
}
