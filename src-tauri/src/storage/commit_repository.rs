//! コミットリポジトリ
//!
//! Commit の永続化を担当する。
//! 一意制約（project_id + branch_name + sha）で重複を防ぐ。

use crate::domain::Commit;
use crate::error::AppResult;
use crate::storage::{read_json, write_json, AppData, BulkUpsertResult, DATA_FILE_NAME};
use std::collections::HashSet;

/// コミットリポジトリ
pub struct CommitRepository;

impl CommitRepository {
    /// 全コミットを取得
    pub fn find_all() -> AppResult<Vec<Commit>> {
        let data = read_json::<AppData>(DATA_FILE_NAME)?;
        Ok(data.map(|d| d.commits).unwrap_or_default())
    }
    
    /// プロジェクト/ブランチでフィルタしたコミットを取得
    pub fn find_by_project_and_branch(project_id: i64, branch_name: &str) -> AppResult<Vec<Commit>> {
        let commits = Self::find_all()?;
        Ok(commits
            .into_iter()
            .filter(|c| c.project_id == project_id && c.branch_name == branch_name)
            .collect())
    }
    
    /// プロジェクトでフィルタしたコミットを取得
    pub fn find_by_project(project_id: i64) -> AppResult<Vec<Commit>> {
        let commits = Self::find_all()?;
        Ok(commits
            .into_iter()
            .filter(|c| c.project_id == project_id)
            .collect())
    }
    
    /// 年でフィルタしたコミットを取得（全プロジェクト横断）
    pub fn find_by_year(year: i32) -> AppResult<Vec<Commit>> {
        let commits = Self::find_all()?;
        Ok(commits
            .into_iter()
            .filter(|c| c.year() == year)
            .collect())
    }
    
    /// 一括挿入（重複スキップ）
    ///
    /// 既存のコミットと重複する（unique_key が同じ）ものはスキップする。
    pub fn bulk_upsert(new_commits: Vec<Commit>) -> AppResult<BulkUpsertResult> {
        let mut data = read_json::<AppData>(DATA_FILE_NAME)?
            .unwrap_or_default();
        
        // 既存のキーをセットに格納
        let existing_keys: HashSet<String> = data.commits
            .iter()
            .map(|c| c.unique_key())
            .collect();
        
        let mut result = BulkUpsertResult::default();
        
        for commit in new_commits {
            let key = commit.unique_key();
            if existing_keys.contains(&key) {
                result.skipped += 1;
            } else {
                data.commits.push(commit);
                result.inserted += 1;
            }
        }
        
        if result.inserted > 0 {
            write_json(DATA_FILE_NAME, &data)?;
        }
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::get_data_file_path;
    use chrono::Utc;
    use std::fs;
    use serial_test::serial;

    fn cleanup_test_data() {
        if let Ok(path) = get_data_file_path(DATA_FILE_NAME) {
            let _ = fs::remove_file(path);
        }
    }

    fn create_test_commit(project_id: i64, branch: &str, sha: &str) -> Commit {
        Commit {
            project_id,
            branch_name: branch.to_string(),
            sha: sha.to_string(),
            message: "test commit".to_string(),
            committed_date_utc: Utc::now(),
            author_name: "Test User".to_string(),
            author_email: Some("test@example.com".to_string()),
            additions: 10,
            deletions: 5,
            stats_missing: false,
        }
    }

    #[test]
    #[serial]
    fn test_bulk_upsert_no_duplicates() {
        cleanup_test_data();
        
        let commits = vec![
            create_test_commit(1, "main", "abc123"),
            create_test_commit(1, "main", "def456"),
        ];
        
        let result = CommitRepository::bulk_upsert(commits).unwrap();
        
        assert_eq!(result.inserted, 2);
        assert_eq!(result.skipped, 0);
        
        cleanup_test_data();
    }

    #[test]
    #[serial]
    fn test_bulk_upsert_with_duplicates() {
        cleanup_test_data();
        
        // 初回挿入
        let commits1 = vec![
            create_test_commit(1, "main", "abc123"),
            create_test_commit(1, "main", "def456"),
        ];
        CommitRepository::bulk_upsert(commits1).unwrap();
        
        // 重複を含む再挿入
        let commits2 = vec![
            create_test_commit(1, "main", "abc123"), // 重複
            create_test_commit(1, "main", "ghi789"), // 新規
        ];
        let result = CommitRepository::bulk_upsert(commits2).unwrap();
        
        assert_eq!(result.inserted, 1);
        assert_eq!(result.skipped, 1);
        
        // 合計 3 件
        let all = CommitRepository::find_all().unwrap();
        assert_eq!(all.len(), 3);
        
        cleanup_test_data();
    }

    #[test]
    fn test_unique_key_constraint() {
        // 同じ project_id + branch_name + sha は重複とみなす
        let c1 = create_test_commit(1, "main", "abc");
        let c2 = create_test_commit(1, "main", "abc");
        let c3 = create_test_commit(1, "develop", "abc"); // ブランチ違い = 別
        let c4 = create_test_commit(2, "main", "abc");    // プロジェクト違い = 別
        
        assert_eq!(c1.unique_key(), c2.unique_key());
        assert_ne!(c1.unique_key(), c3.unique_key());
        assert_ne!(c1.unique_key(), c4.unique_key());
    }
}
