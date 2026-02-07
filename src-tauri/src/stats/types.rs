//! 月次集計の型定義
//!
//! 集計レスポンスとユーザー別データ系列を定義する。

use serde::Serialize;

/// 月次集計レスポンス
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonthlyStatsResponse {
    /// 対象月の配列（1-12）
    pub months: Vec<u32>,
    /// ユーザー別データ系列
    pub series: Vec<UserMonthlySeries>,
}

/// ユーザー別月次データ系列
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMonthlySeries {
    /// ユーザーキー（内部識別用）
    /// NOTE: authorEmail を含む可能性があるが、これは内部キーとして使用し UI には displayName を表示
    pub user_key: String,
    /// 表示名（authorName）
    pub display_name: String,
    /// 月別合計行数（months 配列に対応）
    pub totals: Vec<i64>,
    /// 月別欠損コミット件数（months 配列に対応）
    pub missing_counts: Vec<usize>,
}

impl MonthlyStatsResponse {
    /// 空のレスポンスを作成（12ヶ月分）
    pub fn empty() -> Self {
        Self {
            months: (1..=12).collect(),
            series: Vec::new(),
        }
    }
}

/// ユーザー集計の中間データ
#[derive(Debug, Clone, Default)]
pub struct UserStats {
    /// 表示名
    pub display_name: String,
    /// 月別合計行数（インデックス 0-11 = 1-12月）
    pub monthly_totals: [i64; 12],
    /// 月別欠損件数
    pub monthly_missing: [usize; 12],
}

impl UserStats {
    /// 新規作成
    pub fn new(display_name: &str) -> Self {
        Self {
            display_name: display_name.to_string(),
            ..Default::default()
        }
    }

    /// UserMonthlySeries に変換
    pub fn to_series(&self, user_key: &str) -> UserMonthlySeries {
        UserMonthlySeries {
            user_key: user_key.to_string(),
            display_name: self.display_name.clone(),
            totals: self.monthly_totals.to_vec(),
            missing_counts: self.monthly_missing.to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_stats_to_series() {
        let mut stats = UserStats::new("John Doe");
        stats.monthly_totals[0] = 100; // 1月
        stats.monthly_totals[5] = 200; // 6月
        stats.monthly_missing[0] = 1;

        let series = stats.to_series("john@example.com");

        assert_eq!(series.user_key, "john@example.com");
        assert_eq!(series.display_name, "John Doe");
        assert_eq!(series.totals[0], 100);
        assert_eq!(series.totals[5], 200);
        assert_eq!(series.missing_counts[0], 1);
    }

    #[test]
    fn test_user_key_may_contain_email() {
        // user_key は内部識別用なので email を含んでよい
        let series = UserMonthlySeries {
            user_key: "secret@example.com".to_string(),
            display_name: "John Doe".to_string(),
            totals: vec![0; 12],
            missing_counts: vec![0; 12],
        };

        // display_name には email が含まれない
        assert!(!series.display_name.contains("@"));
    }
}
