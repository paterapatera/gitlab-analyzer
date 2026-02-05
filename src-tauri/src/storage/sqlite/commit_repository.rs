/// コミット情報の SQLite リポジトリ
///
/// コミット情報（commit_id、branch_name、author_email など）を保存・取得します。
/// 一意性制約（project_id, branch_name, sha）を実装し、重複コミットを防止します。
use anyhow::{Context, Result};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commit {
    pub project_id: i32,
    pub branch_name: String,
    pub sha: String,
    pub author_name: String,
    pub author_email: String,
    pub committed_date_utc: String,
    pub additions: i32,
    pub deletions: i32,
}

pub struct CommitRepository;

impl CommitRepository {
    /// コミットをバッチで保存（トランザクション + prepared statement）
    pub fn save_commits(conn: &mut Connection, commits: Vec<Commit>) -> Result<usize> {
        let tx = conn.transaction().context("Failed to start transaction")?;

        let mut stmt = tx.prepare(
            "INSERT OR REPLACE INTO commits
             (project_id, branch_name, sha, author_name, author_email, committed_date_utc, additions, deletions)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        ).context("Failed to prepare insert statement")?;

        let mut inserted_count = 0;

        for commit in commits {
            let rows_affected = stmt
                .execute(rusqlite::params![
                    commit.project_id,
                    commit.branch_name,
                    commit.sha,
                    commit.author_name,
                    commit.author_email,
                    commit.committed_date_utc,
                    commit.additions,
                    commit.deletions,
                ])
                .context("Failed to insert commit")?;

            if rows_affected > 0 {
                inserted_count += rows_affected;
            }
        }

        drop(stmt);
        tx.commit()
            .context("Failed to commit commits transaction")?;

        tracing::info!("Inserted {} commits to database", inserted_count);
        Ok(inserted_count)
    }

    /// プロジェクト内のコミット一覧を取得
    pub fn get_commits_by_project(conn: &Connection, project_id: i32) -> Result<Vec<Commit>> {
        let mut stmt = conn.prepare(
            "SELECT project_id, branch_name, sha, author_name, author_email, committed_date_utc, additions, deletions
             FROM commits WHERE project_id = ? ORDER BY committed_date_utc DESC"
        ).context("Failed to prepare commits query")?;

        let commits = stmt
            .query_map(rusqlite::params![project_id], |row| {
                Ok(Commit {
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
            .context("Failed to query commits")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect commits")?;

        Ok(commits)
    }

    /// ブランチ内のコミット一覧を取得
    pub fn get_commits_by_branch(
        conn: &Connection,
        project_id: i32,
        branch_name: &str,
    ) -> Result<Vec<Commit>> {
        let mut stmt = conn.prepare(
            "SELECT project_id, branch_name, sha, author_name, author_email, committed_date_utc, additions, deletions
             FROM commits WHERE project_id = ? AND branch_name = ? ORDER BY committed_date_utc DESC"
        ).context("Failed to prepare commits by branch query")?;

        let commits = stmt
            .query_map(rusqlite::params![project_id, branch_name], |row| {
                Ok(Commit {
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
            .context("Failed to query commits by branch")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect commits by branch")?;

        Ok(commits)
    }

    /// 期間内のコミット一覧を取得
    pub fn get_commits_by_date_range(
        conn: &Connection,
        project_id: i32,
        start_date: &str,
        end_date: &str,
    ) -> Result<Vec<Commit>> {
        let mut stmt = conn.prepare(
            "SELECT project_id, branch_name, sha, author_name, author_email, committed_date_utc, additions, deletions
             FROM commits
             WHERE project_id = ? AND committed_date_utc >= ? AND committed_date_utc < ?
             ORDER BY committed_date_utc DESC"
        ).context("Failed to prepare commits by date range query")?;

        let commits = stmt
            .query_map(rusqlite::params![project_id, start_date, end_date], |row| {
                Ok(Commit {
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
            .context("Failed to query commits by date range")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect commits by date range")?;

        Ok(commits)
    }

    /// 著者ごとのコミット統計を取得（月次レベル）
    pub fn get_monthly_stats_by_author(
        conn: &Connection,
        project_id: i32,
        author_email: &str,
    ) -> Result<Vec<(String, i32, i32, i32)>> {
        let mut stmt = conn
            .prepare(
                "SELECT
                substr(committed_date_utc, 1, 7) as month,
                COUNT(*) as commit_count,
                SUM(additions) as total_additions,
                SUM(deletions) as total_deletions
             FROM commits
             WHERE project_id = ? AND author_email = ?
             GROUP BY month
             ORDER BY month DESC",
            )
            .context("Failed to prepare monthly stats query")?;

        let stats = stmt
            .query_map(rusqlite::params![project_id, author_email], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i32>(1)?,
                    row.get::<_, i32>(2)?,
                    row.get::<_, i32>(3)?,
                ))
            })
            .context("Failed to query monthly stats")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect monthly stats")?;

        Ok(stats)
    }

    /// プロジェクト全体の月次統計を取得
    pub fn get_monthly_stats_project(
        conn: &Connection,
        project_id: i32,
    ) -> Result<Vec<(String, i32, i32, i32)>> {
        let mut stmt = conn
            .prepare(
                "SELECT
                substr(committed_date_utc, 1, 7) as month,
                COUNT(*) as commit_count,
                SUM(additions) as total_additions,
                SUM(deletions) as total_deletions
             FROM commits
             WHERE project_id = ?
             GROUP BY month
             ORDER BY month DESC",
            )
            .context("Failed to prepare project monthly stats query")?;

        let stats = stmt
            .query_map(rusqlite::params![project_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i32>(1)?,
                    row.get::<_, i32>(2)?,
                    row.get::<_, i32>(3)?,
                ))
            })
            .context("Failed to query project monthly stats")?
            .collect::<Result<Vec<_>, _>>()
            .context("Failed to collect project monthly stats")?;

        Ok(stats)
    }

    /// プロジェクト内のコミット総数を取得
    pub fn count_commits_by_project(conn: &Connection, project_id: i32) -> Result<i32> {
        let count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM commits WHERE project_id = ?",
                rusqlite::params![project_id],
                |row| row.get(0),
            )
            .context("Failed to count commits")?;

        Ok(count)
    }

    /// 単一コミットの取得
    pub fn get_commit(
        conn: &Connection,
        project_id: i32,
        branch_name: &str,
        sha: &str,
    ) -> Result<Option<Commit>> {
        let mut stmt = conn.prepare(
            "SELECT project_id, branch_name, sha, author_name, author_email, committed_date_utc, additions, deletions
             FROM commits WHERE project_id = ? AND branch_name = ? AND sha = ?"
        ).context("Failed to prepare commit query")?;

        let result = stmt.query_row(rusqlite::params![project_id, branch_name, sha], |row| {
            Ok(Commit {
                project_id: row.get(0)?,
                branch_name: row.get(1)?,
                sha: row.get(2)?,
                author_name: row.get(3)?,
                author_email: row.get(4)?,
                committed_date_utc: row.get(5)?,
                additions: row.get(6)?,
                deletions: row.get(7)?,
            })
        });

        match result {
            Ok(commit) => Ok(Some(commit)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(anyhow::anyhow!("Failed to query commit: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_connection() -> rusqlite::Connection {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();

        // テーブル作成
        conn.execute(
            "CREATE TABLE commits (
                project_id INTEGER NOT NULL,
                branch_name TEXT NOT NULL,
                sha TEXT NOT NULL,
                author_name TEXT NOT NULL,
                author_email TEXT NOT NULL,
                committed_date_utc TEXT NOT NULL,
                additions INTEGER NOT NULL,
                deletions INTEGER NOT NULL,
                PRIMARY KEY (project_id, branch_name, sha)
            )",
            [],
        )
        .unwrap();

        conn
    }

    #[test]
    fn test_save_commits() {
        let mut conn = create_test_connection();

        let commits = vec![Commit {
            project_id: 1,
            branch_name: "main".to_string(),
            sha: "abc123".to_string(),
            author_name: "Test Author".to_string(),
            author_email: "test@example.com".to_string(),
            committed_date_utc: "2024-01-01T00:00:00Z".to_string(),
            additions: 10,
            deletions: 5,
        }];

        let result = CommitRepository::save_commits(&mut conn, commits);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }
}
