//! コミットリポジトリ（SQLite ベース）
//!
//! Commit の永続化を SQLite で行います。

use crate::domain::Commit;
use crate::error::{AppError, AppResult};
use crate::storage::{sqlite, BulkUpsertResult};

/// コミットリポジトリ
pub struct CommitRepository;

impl CommitRepository {
    /// 全コミットを取得
    pub fn find_all() -> AppResult<Vec<Commit>> {
        // NOTE: SQLite では全コミット取得は非効率なため、
        // この実装は基本的に使用しない前提
        Err(AppError::Storage(
            "find_all is not supported for SQLite. Use find_by_project or find_by_year instead."
                .to_string(),
        ))
    }

    /// プロジェクト/ブランチでフィルタしたコミットを取得
    pub fn find_by_project_and_branch(
        project_id: i64,
        branch_name: &str,
    ) -> AppResult<Vec<Commit>> {
        let conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        let sqlite_commits =
            sqlite::CommitRepository::get_commits_by_branch(&conn, project_id as i32, branch_name)
                .map_err(|e| AppError::Storage(e.to_string()))?;

        // SQLite の Commit 型をドメインの Commit 型に変換
        let commits = sqlite_commits
            .into_iter()
            .map(Self::convert_from_sqlite)
            .collect::<AppResult<Vec<_>>>()?;

        Ok(commits)
    }

    /// プロジェクトでフィルタしたコミットを取得
    pub fn find_by_project(project_id: i64) -> AppResult<Vec<Commit>> {
        let conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        let sqlite_commits =
            sqlite::CommitRepository::get_commits_by_project(&conn, project_id as i32)
                .map_err(|e| AppError::Storage(e.to_string()))?;

        // SQLite の Commit 型をドメインの Commit 型に変換
        let commits = sqlite_commits
            .into_iter()
            .map(Self::convert_from_sqlite)
            .collect::<AppResult<Vec<_>>>()?;

        Ok(commits)
    }

    /// 指定ブランチの最終コミット時刻を取得
    pub fn get_last_commit_time(project_id: i64, branch_name: &str) -> AppResult<Option<String>> {
        let conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        let result = conn.query_row(
            "SELECT MAX(committed_date_utc) FROM commits WHERE project_id = ?1 AND branch_name = ?2",
            rusqlite::params![project_id, branch_name],
            |row| row.get::<_, Option<String>>(0),
        );

        match result {
            Ok(value) => Ok(value),
            Err(e) => Err(AppError::Storage(e.to_string())),
        }
    }

    /// 年でフィルタしたコミットを取得（全プロジェクト横断）
    pub fn find_by_year(year: i32) -> AppResult<Vec<Commit>> {
        let conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        // NOTE: SQLite の実装で年フィルタのメソッドが必要
        // 現時点では簡易的に範囲クエリで検索
        let start_date = format!("{}-01-01T00:00:00Z", year);
        let end_date = format!("{}-01-01T00:00:00Z", year + 1);

        // 全プロジェクトから取得
        let mut stmt = conn
            .prepare(
                "SELECT project_id, branch_name, sha, author_name, author_email, 
                 committed_date_utc, additions, deletions
                 FROM commits
                 WHERE committed_date_utc >= ? AND committed_date_utc < ?
                 ORDER BY committed_date_utc DESC",
            )
            .map_err(|e| AppError::Storage(e.to_string()))?;

        let commits = stmt
            .query_map(rusqlite::params![start_date, end_date], |row| {
                Ok(sqlite::commit_repository::Commit {
                    project_id: row.get(0)?,
                    branch_name: row.get(1)?,
                    sha: row.get(2)?,
                    author_name: row.get(3)?,
                    author_email: row.get(4)?,
                    committed_date_utc: row.get(5)?,
                    additions: row.get(6)?,
                    deletions: row.get(7)?,
                })
            })
            .map_err(|e| AppError::Storage(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        let commits = commits
            .into_iter()
            .map(Self::convert_from_sqlite)
            .collect::<AppResult<Vec<_>>>()?;

        Ok(commits)
    }

    /// 一括挿入（重複スキップ）
    pub fn bulk_upsert(new_commits: Vec<Commit>) -> AppResult<BulkUpsertResult> {
        let mut conn = sqlite::DatabaseConnection::create_connection()
            .map_err(|e| AppError::Storage(e.to_string()))?;

        // ドメインの Commit 型を SQLite の Commit 型に変換
        let sqlite_commits = new_commits
            .into_iter()
            .map(Self::convert_to_sqlite)
            .collect::<AppResult<Vec<_>>>()?;

        // プロジェクト ID を取得（保存後に使用）
        let project_id = sqlite_commits.first().map(|c| c.project_id).unwrap_or(0);
        let total_input = sqlite_commits.len();

        // SQLite の save_commits は INSERT OR REPLACE を使うため、
        // 既存のレコードは更新される（実質的には重複スキップと同等）
        let rows_before = sqlite::CommitRepository::count_commits_by_project(&conn, project_id)
            .map_err(|e| AppError::Storage(e.to_string()))?;

        sqlite::CommitRepository::save_commits(&mut conn, sqlite_commits)
            .map_err(|e| AppError::Storage(e.to_string()))?;

        let rows_after = sqlite::CommitRepository::count_commits_by_project(&conn, project_id)
            .map_err(|e| AppError::Storage(e.to_string()))?;

        let inserted = (rows_after - rows_before) as usize;
        let skipped = total_input.saturating_sub(inserted);

        Ok(BulkUpsertResult { inserted, skipped })
    }

    /// SQLite の Commit をドメインの Commit に変換
    fn convert_from_sqlite(c: sqlite::commit_repository::Commit) -> AppResult<Commit> {
        let committed_date_utc = chrono::DateTime::parse_from_rfc3339(&c.committed_date_utc)
            .map_err(|e| AppError::Storage(format!("Invalid date format: {}", e)))?
            .with_timezone(&chrono::Utc);

        Ok(Commit {
            project_id: c.project_id as i64,
            branch_name: c.branch_name,
            sha: c.sha,
            message: String::new(), // NOTE: SQLite スキーマに message フィールドがない
            committed_date_utc,
            author_name: c.author_name,
            author_email: Some(c.author_email),
            additions: c.additions as i64,
            deletions: c.deletions as i64,
            stats_missing: false, // NOTE: SQLite スキーマに stats_missing フィールドがない
        })
    }

    /// ドメインの Commit を SQLite の Commit に変換
    fn convert_to_sqlite(c: Commit) -> AppResult<sqlite::commit_repository::Commit> {
        Ok(sqlite::commit_repository::Commit {
            project_id: c.project_id as i32,
            branch_name: c.branch_name,
            sha: c.sha,
            author_name: c.author_name,
            author_email: c.author_email.unwrap_or_default(),
            committed_date_utc: c.committed_date_utc.to_rfc3339(),
            additions: c.additions as i32,
            deletions: c.deletions as i32,
        })
    }
}
