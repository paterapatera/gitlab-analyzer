//! セキュアログユーティリティ
//!
//! アクセストークンやメールアドレスがログに出力されないようにする。
//! FR-018/FR-019 の要件を満たす。

use tracing_subscriber::{fmt, EnvFilter};

/// ログの初期化
///
/// 環境変数 `RUST_LOG` でレベル制御可能。デフォルトは `info`。
pub fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_level(true)
        .init();
}

/// 機密情報をマスクした文字列を返す
///
/// トークンやメールアドレスを安全にログ出力するために使用する。
///
/// # Examples
/// ```
/// use gitlab_analyzer_lib::logging::mask_sensitive;
/// assert_eq!(mask_sensitive("glpat-xxxx1234"), "glp***");
/// assert_eq!(mask_sensitive("short"), "***");
/// ```
pub fn mask_sensitive(value: &str) -> String {
    if value.len() <= 6 {
        "***".to_string()
    } else {
        format!("{}***", &value[..3])
    }
}

/// メールアドレスをマスクした文字列を返す
///
/// `user@example.com` → `u***@e***.com`
pub fn mask_email(email: &str) -> String {
    if let Some(at_pos) = email.find('@') {
        let local = &email[..at_pos];
        let domain = &email[at_pos + 1..];
        
        let masked_local = if local.is_empty() {
            "***".to_string()
        } else {
            format!("{}***", &local[..1.min(local.len())])
        };
        
        let masked_domain = if domain.is_empty() {
            "***".to_string()
        } else if let Some(dot_pos) = domain.rfind('.') {
            let domain_name = &domain[..dot_pos];
            let tld = &domain[dot_pos..];
            if domain_name.is_empty() {
                format!("***{}", tld)
            } else {
                format!("{}***{}", &domain_name[..1.min(domain_name.len())], tld)
            }
        } else {
            format!("{}***", &domain[..1.min(domain.len())])
        };
        
        format!("{}@{}", masked_local, masked_domain)
    } else {
        mask_sensitive(email)
    }
}

/// ログ出力用のトレースマクロ（機密情報なし）
///
/// 通常のログ出力に使用する。トークンやメールを含む場合は
/// `mask_sensitive` や `mask_email` でマスクしてから渡すこと。
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        tracing::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        tracing::error!($($arg)*)
    };
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        tracing::debug!($($arg)*)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mask_sensitive_token() {
        // トークンがマスクされる
        assert_eq!(mask_sensitive("glpat-xxxxxxxxxxxx"), "glp***");
        assert_eq!(mask_sensitive("abc"), "***");
        assert_eq!(mask_sensitive(""), "***");
    }

    #[test]
    fn test_mask_email() {
        assert_eq!(mask_email("user@example.com"), "u***@e***.com");
        assert_eq!(mask_email("a@b.co"), "a***@b***.co");
        // エッジケース
        assert_eq!(mask_email("@domain.com"), "***@d***.com");
    }

    #[test]
    fn test_mask_email_no_at_sign() {
        // @ がない場合は通常のマスク
        assert_eq!(mask_email("notanemail"), "not***");
    }

    #[test]
    fn test_token_not_in_masked_output() {
        let token = "glpat-secret-token-12345";
        let masked = mask_sensitive(token);
        // 元のトークンが含まれていないこと
        assert!(!masked.contains("secret"));
        assert!(!masked.contains("token"));
        assert!(!masked.contains("12345"));
    }

    #[test]
    fn test_email_not_in_masked_output() {
        let email = "secret.user@private-domain.com";
        let masked = mask_email(email);
        // 元のメールアドレスの詳細が含まれていないこと
        assert!(!masked.contains("secret"));
        assert!(!masked.contains("user"));
        assert!(!masked.contains("private"));
        assert!(!masked.contains("domain"));
    }
}
