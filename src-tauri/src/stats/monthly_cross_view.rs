//! 横断ビュー月次集計
//!
//! 全プロジェクト横断で月次コミット行数を集計する。

use crate::domain::Commit;
use crate::stats::{MonthlyStatsResponse, UserStats};
use std::collections::HashMap;

/// 横断ビューの月次集計を行う
///
/// # Arguments
/// * `commits` - 対象コミット一覧（既に年でフィルタ済み）
/// * `user_keys` - フィルタするユーザーキー（空の場合は全ユーザー）
pub fn aggregate_cross_view(commits: &[Commit], user_keys: &[String]) -> MonthlyStatsResponse {
    let mut user_stats_map: HashMap<String, UserStats> = HashMap::new();

    for commit in commits {
        let user_key = commit.user_key();

        // ユーザーフィルタ
        if !user_keys.is_empty() && !user_keys.contains(&user_key) {
            continue;
        }

        let month_index = (commit.month() as usize).saturating_sub(1);
        if month_index >= 12 {
            continue;
        }

        let stats = user_stats_map
            .entry(user_key.clone())
            .or_insert_with(|| UserStats::new(commit.display_name()));

        stats.monthly_totals[month_index] += commit.total_lines();
        if commit.stats_missing {
            stats.monthly_missing[month_index] += 1;
        }
    }

    let series: Vec<_> = user_stats_map
        .iter()
        .map(|(key, stats)| stats.to_series(key))
        .collect();

    MonthlyStatsResponse {
        months: (1..=12).collect(),
        series,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn create_commit(
        project_id: i64,
        month: u32,
        user: &str,
        additions: i64,
        missing: bool,
    ) -> Commit {
        Commit {
            project_id,
            branch_name: "main".to_string(),
            sha: format!("sha-{}-{}-{}", project_id, user, month),
            message: "test".to_string(),
            committed_date_utc: Utc.with_ymd_and_hms(2026, month, 15, 12, 0, 0).unwrap(),
            author_name: user.to_string(),
            author_email: Some(format!("{}@example.com", user.to_lowercase())),
            additions,
            deletions: 0,
            stats_missing: missing,
        }
    }

    #[test]
    fn test_cross_view_aggregates_all_projects() {
        let commits = vec![
            create_commit(1, 1, "Alice", 100, false),
            create_commit(2, 1, "Alice", 50, false), // 別プロジェクト
            create_commit(1, 1, "Bob", 80, false),
        ];

        let result = aggregate_cross_view(&commits, &[]);

        // Alice の 1月は project 1 + project 2 の合計
        let alice = result
            .series
            .iter()
            .find(|s| s.display_name == "Alice")
            .unwrap();
        assert_eq!(alice.totals[0], 150);
    }

    #[test]
    fn test_cross_view_missing_count() {
        let commits = vec![
            create_commit(1, 2, "Alice", 0, true),
            create_commit(2, 2, "Alice", 0, true),
            create_commit(1, 2, "Alice", 100, false),
        ];

        let result = aggregate_cross_view(&commits, &[]);

        let alice = result
            .series
            .iter()
            .find(|s| s.display_name == "Alice")
            .unwrap();
        // 2月に2件欠損
        assert_eq!(alice.missing_counts[1], 2);
    }
}
