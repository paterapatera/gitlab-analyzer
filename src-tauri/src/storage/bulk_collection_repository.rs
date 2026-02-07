//! 一括収集リポジトリ（SQLite ベース）
//!
//! 一括収集の実行・対象・結果の状態を SQLite に保存する。

use crate::error::{AppError, AppResult};
use crate::storage::model::{BulkCollectionStatus, BulkCollectionTargetResult};
use crate::storage::sqlite;
use rusqlite::params;
use uuid::Uuid;

/// 一括収集の実行を開始
pub fn start_run(total_targets: usize) -> AppResult<String> {
    let conn = sqlite::DatabaseConnection::create_connection()
        .map_err(|e| AppError::Storage(e.to_string()))?;

    start_run_with_connection(&conn, total_targets)
}

/// 対象を pending 状態で登録
pub fn register_targets(run_id: &str, targets: &[(i64, String)]) -> AppResult<()> {
    let mut conn = sqlite::DatabaseConnection::create_connection()
        .map_err(|e| AppError::Storage(e.to_string()))?;

    register_targets_with_connection(&mut conn, run_id, targets)
}

/// 対象の処理結果を記録
pub fn record_target_result(
    run_id: &str,
    project_id: i64,
    branch_name: &str,
    success: bool,
    new_commits_count: Option<usize>,
    error_message: Option<&str>,
) -> AppResult<()> {
    let conn = sqlite::DatabaseConnection::create_connection()
        .map_err(|e| AppError::Storage(e.to_string()))?;

    record_target_result_with_connection(
        &conn,
        run_id,
        project_id,
        branch_name,
        success,
        new_commits_count,
        error_message,
    )
}

/// 実行を完了状態にする
pub fn complete_run(run_id: &str) -> AppResult<()> {
    let conn = sqlite::DatabaseConnection::create_connection()
        .map_err(|e| AppError::Storage(e.to_string()))?;
    complete_run_with_connection(&conn, run_id)
}

/// 実行をキャンセル状態にする
pub fn cancel_run(run_id: &str) -> AppResult<()> {
    let conn = sqlite::DatabaseConnection::create_connection()
        .map_err(|e| AppError::Storage(e.to_string()))?;
    cancel_run_with_connection(&conn, run_id)
}

/// 実行を再開状態に戻す
pub fn resume_run(run_id: &str) -> AppResult<()> {
    let conn = sqlite::DatabaseConnection::create_connection()
        .map_err(|e| AppError::Storage(e.to_string()))?;
    resume_run_with_connection(&conn, run_id)
}

/// 実行中の一括収集が存在するか
pub fn has_running_run() -> AppResult<bool> {
    let conn = sqlite::DatabaseConnection::create_connection()
        .map_err(|e| AppError::Storage(e.to_string()))?;
    has_running_run_with_connection(&conn)
}

/// 再開可能な実行IDを取得
pub fn get_latest_resumable_run() -> AppResult<Option<String>> {
    let conn = sqlite::DatabaseConnection::create_connection()
        .map_err(|e| AppError::Storage(e.to_string()))?;
    get_latest_resumable_run_with_connection(&conn)
}

/// 一括収集の対象（収集履歴あり）を取得
pub fn get_collection_targets() -> AppResult<Vec<(i64, String)>> {
    let conn = sqlite::DatabaseConnection::create_connection()
        .map_err(|e| AppError::Storage(e.to_string()))?;
    get_collection_targets_with_connection(&conn)
}

/// pending 状態の対象を取得（再開用）
pub fn get_pending_targets(run_id: &str) -> AppResult<Vec<(i64, String)>> {
    get_targets_by_status(run_id, "pending")
}

/// failed 状態の対象を取得（再試行用）
pub fn get_failed_targets(run_id: &str) -> AppResult<Vec<(i64, String)>> {
    get_targets_by_status(run_id, "failed")
}

fn get_targets_by_status(run_id: &str, status: &str) -> AppResult<Vec<(i64, String)>> {
    let conn = sqlite::DatabaseConnection::create_connection()
        .map_err(|e| AppError::Storage(e.to_string()))?;
    get_targets_by_status_with_connection(&conn, run_id, status)
}

/// 実行状態を取得（必要に応じて結果詳細も含める）
pub fn get_status(run_id: &str, include_results: bool) -> AppResult<BulkCollectionStatus> {
    let conn = sqlite::DatabaseConnection::create_connection()
        .map_err(|e| AppError::Storage(e.to_string()))?;
    get_status_with_connection(&conn, run_id, include_results)
}

fn get_results(conn: &rusqlite::Connection, run_id: &str) -> AppResult<Vec<BulkCollectionTargetResult>> {
    let mut stmt = conn
        .prepare(
            "SELECT project_id, branch_name, status, new_commits_count, error_message, processed_at_utc
             FROM bulk_collection_results
             WHERE run_id = ?1
             ORDER BY project_id, branch_name",
        )
        .map_err(|e| AppError::Storage(e.to_string()))?;

    let results = stmt
        .query_map(params![run_id], |row| {
            Ok(BulkCollectionTargetResult {
                project_id: row.get(0)?,
                branch_name: row.get(1)?,
                status: row.get(2)?,
                new_commits_count: row.get(3)?,
                error_message: row.get(4)?,
                processed_at: row.get(5)?,
            })
        })
        .map_err(|e| AppError::Storage(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::Storage(e.to_string()))?;

    Ok(results)
}

pub(crate) fn start_run_with_connection(
    conn: &rusqlite::Connection,
    total_targets: usize,
) -> AppResult<String> {
    let run_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO bulk_collection_runs (run_id, started_at_utc, status, total_targets)
         VALUES (?1, ?2, 'running', ?3)",
        params![run_id, now, total_targets as i64],
    )
    .map_err(|e| AppError::Storage(e.to_string()))?;

    Ok(run_id)
}

pub(crate) fn register_targets_with_connection(
    conn: &mut rusqlite::Connection,
    run_id: &str,
    targets: &[(i64, String)],
) -> AppResult<()> {
    let tx = conn
        .transaction()
        .map_err(|e| AppError::Storage(e.to_string()))?;

    for (project_id, branch_name) in targets {
        tx.execute(
            "INSERT INTO bulk_collection_results (run_id, project_id, branch_name, status)
             VALUES (?1, ?2, ?3, 'pending')",
            params![run_id, project_id, branch_name],
        )
        .map_err(|e| AppError::Storage(e.to_string()))?;
    }

    tx.commit().map_err(|e| AppError::Storage(e.to_string()))?;
    Ok(())
}

pub(crate) fn record_target_result_with_connection(
    conn: &rusqlite::Connection,
    run_id: &str,
    project_id: i64,
    branch_name: &str,
    success: bool,
    new_commits_count: Option<usize>,
    error_message: Option<&str>,
) -> AppResult<()> {
    let now = chrono::Utc::now().to_rfc3339();
    let status = if success { "success" } else { "failed" };

    conn.execute(
        "UPDATE bulk_collection_results
         SET status = ?1, new_commits_count = ?2, error_message = ?3, processed_at_utc = ?4
         WHERE run_id = ?5 AND project_id = ?6 AND branch_name = ?7",
        params![
            status,
            new_commits_count.map(|count| count as i64),
            error_message,
            now,
            run_id,
            project_id,
            branch_name,
        ],
    )
    .map_err(|e| AppError::Storage(e.to_string()))?;

    let count_field = if success { "success_count" } else { "failed_count" };
    conn.execute(
        &format!(
            "UPDATE bulk_collection_runs
             SET completed_count = completed_count + 1, {field} = {field} + 1
             WHERE run_id = ?1",
            field = count_field
        ),
        params![run_id],
    )
    .map_err(|e| AppError::Storage(e.to_string()))?;

    Ok(())
}

pub(crate) fn complete_run_with_connection(
    conn: &rusqlite::Connection,
    run_id: &str,
) -> AppResult<()> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE bulk_collection_runs
         SET status = 'completed', completed_at_utc = ?1
         WHERE run_id = ?2 AND status = 'running'",
        params![now, run_id],
    )
    .map_err(|e| AppError::Storage(e.to_string()))?;
    Ok(())
}

pub(crate) fn cancel_run_with_connection(
    conn: &rusqlite::Connection,
    run_id: &str,
) -> AppResult<()> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE bulk_collection_runs
         SET status = 'cancelled', completed_at_utc = ?1
         WHERE run_id = ?2 AND status = 'running'",
        params![now, run_id],
    )
    .map_err(|e| AppError::Storage(e.to_string()))?;
    Ok(())
}

pub(crate) fn resume_run_with_connection(
    conn: &rusqlite::Connection,
    run_id: &str,
) -> AppResult<()> {
    conn.execute(
        "UPDATE bulk_collection_runs
         SET status = 'running', completed_at_utc = NULL
         WHERE run_id = ?1 AND status = 'cancelled'",
        params![run_id],
    )
    .map_err(|e| AppError::Storage(e.to_string()))?;
    Ok(())
}

pub(crate) fn get_latest_resumable_run_with_connection(
    conn: &rusqlite::Connection,
) -> AppResult<Option<String>> {
    let result = conn.query_row(
        "SELECT run_id FROM bulk_collection_runs
         WHERE status = 'cancelled' AND completed_count < total_targets
         ORDER BY started_at_utc DESC
         LIMIT 1",
        [],
        |row| row.get::<_, String>(0),
    );

    match result {
        Ok(run_id) => Ok(Some(run_id)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(AppError::Storage(e.to_string())),
    }
}

pub(crate) fn has_running_run_with_connection(conn: &rusqlite::Connection) -> AppResult<bool> {
    let is_running: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM bulk_collection_runs WHERE status = 'running')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| AppError::Storage(e.to_string()))?;

    Ok(is_running)
}

pub(crate) fn get_collection_targets_with_connection(
    conn: &rusqlite::Connection,
) -> AppResult<Vec<(i64, String)>> {
    let mut stmt = conn
        .prepare(
            "SELECT DISTINCT project_id, branch_name
             FROM commits
             ORDER BY project_id, branch_name",
        )
        .map_err(|e| AppError::Storage(e.to_string()))?;

    let targets = stmt
        .query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)))
        .map_err(|e| AppError::Storage(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::Storage(e.to_string()))?;

    Ok(targets)
}

pub(crate) fn get_targets_by_status_with_connection(
    conn: &rusqlite::Connection,
    run_id: &str,
    status: &str,
) -> AppResult<Vec<(i64, String)>> {
    let mut stmt = conn
        .prepare(
            "SELECT project_id, branch_name
             FROM bulk_collection_results
             WHERE run_id = ?1 AND status = ?2
             ORDER BY project_id, branch_name",
        )
        .map_err(|e| AppError::Storage(e.to_string()))?;

    let targets = stmt
        .query_map(params![run_id, status], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| AppError::Storage(e.to_string()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::Storage(e.to_string()))?;

    Ok(targets)
}

pub(crate) fn get_status_with_connection(
    conn: &rusqlite::Connection,
    run_id: &str,
    include_results: bool,
) -> AppResult<BulkCollectionStatus> {
    let mut stmt = conn
        .prepare(
            "SELECT run_id, status, total_targets, completed_count, success_count, failed_count,
                    started_at_utc, completed_at_utc
             FROM bulk_collection_runs
             WHERE run_id = ?1",
        )
        .map_err(|e| AppError::Storage(e.to_string()))?;

    let mut status = stmt
        .query_row(params![run_id], |row| {
            Ok(BulkCollectionStatus {
                run_id: row.get(0)?,
                status: row.get(1)?,
                total_targets: row.get(2)?,
                completed_count: row.get(3)?,
                success_count: row.get(4)?,
                failed_count: row.get(5)?,
                started_at: row.get(6)?,
                completed_at: row.get(7)?,
                results: Vec::new(),
            })
        })
        .map_err(|e| AppError::Storage(e.to_string()))?;

    if include_results {
        status.results = get_results(conn, run_id)?;
    }

    Ok(status)
}
