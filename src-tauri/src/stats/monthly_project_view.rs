//! プロジェクトビュー月次集計
//!
//! 特定プロジェクト/ブランチの月次コミット行数を集計する。

use crate::domain::Commit;
use crate::stats::{MonthlyStatsResponse, UserStats};
use std::collections::HashMap;

/// プロジェクトビューの月次集計を行う
///
/// # Arguments
/// * `commits` - 対象コミット一覧（既にプロジェクト/ブランチ/年でフィルタ済み）
/// * `user_keys` - フィルタするユーザーキー（空の場合は全ユーザー）
pub fn aggregate_project_view(commits: &[Commit], user_keys: &[String]) -> MonthlyStatsResponse {
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
        month: u32,
        user: &str,
        email: Option<&str>,
        additions: i64,
        deletions: i64,
        missing: bool,
    ) -> Commit {
        Commit {
            project_id: 1,
            branch_name: "main".to_string(),
            sha: format!("sha-{}-{}", user, month),
            message: "test".to_string(),
            committed_date_utc: Utc.with_ymd_and_hms(2026, month, 15, 12, 0, 0).unwrap(),
            author_name: user.to_string(),
            author_email: email.map(|s| s.to_string()),
            additions,
            deletions,
            stats_missing: missing,
        }
    }

    #[test]
    fn test_aggregate_simple() {
        let commits = vec![
            create_commit(1, "Alice", Some("alice@example.com"), 100, 50, false),
            create_commit(1, "Bob", None, 80, 20, false),
            create_commit(2, "Alice", Some("alice@example.com"), 60, 40, false),
        ];

        let result = aggregate_project_view(&commits, &[]);

        assert_eq!(result.months.len(), 12);
        assert_eq!(result.series.len(), 2);

        // Alice の集計
        let alice = result
            .series
            .iter()
            .find(|s| s.display_name == "Alice")
            .unwrap();
        assert_eq!(alice.totals[0], 150); // 1月: 100+50
        assert_eq!(alice.totals[1], 100); // 2月: 60+40

        // Bob の集計
        let bob = result
            .series
            .iter()
            .find(|s| s.display_name == "Bob")
            .unwrap();
        assert_eq!(bob.totals[0], 100); // 1月: 80+20
    }

    #[test]
    fn test_aggregate_with_user_filter() {
        let commits = vec![
            create_commit(1, "Alice", Some("alice@example.com"), 100, 0, false),
            create_commit(1, "Bob", None, 50, 0, false),
        ];

        let result = aggregate_project_view(&commits, &["alice@example.com".to_string()]);

        // Alice のみ
        assert_eq!(result.series.len(), 1);
        assert_eq!(result.series[0].display_name, "Alice");
    }

    #[test]
    fn test_aggregate_missing_stats_count() {
        let commits = vec![
            create_commit(3, "Alice", None, 0, 0, true), // stats 欠損
            create_commit(3, "Alice", None, 100, 0, false),
        ];

        let result = aggregate_project_view(&commits, &[]);

        let alice = result
            .series
            .iter()
            .find(|s| s.display_name == "Alice")
            .unwrap();
        assert_eq!(alice.missing_counts[2], 1); // 3月に1件欠損
    }

    #[test]
    fn test_month_is_utc_based() {
        // UTC で 2026-06-30 23:59:59 のコミット
        let commit = Commit {
            project_id: 1,
            branch_name: "main".to_string(),
            sha: "test".to_string(),
            message: "test".to_string(),
            committed_date_utc: Utc.with_ymd_and_hms(2026, 6, 30, 23, 59, 59).unwrap(),
            author_name: "Test".to_string(),
            author_email: None,
            additions: 10,
            deletions: 0,
            stats_missing: false,
        };

        let result = aggregate_project_view(&[commit], &[]);

        let test_user = result
            .series
            .iter()
            .find(|s| s.display_name == "Test")
            .unwrap();
        // 6月にカウントされる（インデックス 5）
        assert_eq!(test_user.totals[5], 10);
    }
}
