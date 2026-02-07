//! コマンド: 一括コミット収集
//!
//! 収集履歴のある全対象を順次処理し、結果を保存する。

use crate::commands::commits_collect::{collect_commits_inner, CollectCommitsRequest};
use crate::error::{AppError, AppResult};
use crate::storage::bulk_collection_repository::{
    get_collection_targets_with_connection, get_latest_resumable_run_with_connection,
    get_status_with_connection, get_targets_by_status_with_connection,
    register_targets_with_connection, resume_run_with_connection, start_run_with_connection,
};
use crate::storage::sqlite::DatabaseConnection;
use crate::storage::{bulk_collection_repository, CommitRepository};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};
use tracing::{info, warn};

static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

/// 一括収集開始レスポンス
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkCollectionStarted {
    pub run_id: String,
    pub total_targets: i64,
}

/// 進捗イベントのペイロード
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct BulkCollectionProgress {
    pub run_id: String,
    pub total_targets: i64,
    pub completed_count: i64,
    pub success_count: i64,
    pub failed_count: i64,
    pub current_target: Option<TargetInfo>,
}

/// 現在処理中の対象
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct TargetInfo {
    pub project_id: i64,
    pub branch_name: String,
}

/// 失敗対象再試行のリクエスト
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RetryFailedRequest {
    pub run_id: String,
}

/// 一括収集を開始
#[tauri::command]
pub async fn collect_commits_bulk(app: AppHandle) -> Result<BulkCollectionStarted, String> {
    let context = prepare_bulk_collection_start().map_err(|e| e.user_message())?;

    CANCEL_FLAG.store(false, Ordering::SeqCst);

    let run_id = context.run_id.clone();
    let total_targets = context.total_targets;
    let app_clone = app.clone();

    tokio::spawn(async move {
        process_bulk_collection(app_clone, context).await;
    });

    Ok(BulkCollectionStarted {
        run_id,
        total_targets,
    })
}

/// 一括収集をキャンセル
#[tauri::command]
pub fn cancel_bulk_collection() -> Result<(), String> {
    CANCEL_FLAG.store(true, Ordering::SeqCst);
    Ok(())
}

pub(crate) fn is_cancel_requested() -> bool {
    CANCEL_FLAG.load(Ordering::SeqCst)
}

pub(crate) fn reset_cancel_flag() {
    CANCEL_FLAG.store(false, Ordering::SeqCst);
}

/// 一括収集の状態を取得
#[tauri::command]
#[allow(non_snake_case)]
pub fn get_bulk_collection_status(
    runId: String,
) -> Result<crate::storage::model::BulkCollectionStatus, String> {
    bulk_collection_repository::get_status(&runId, true).map_err(|e| e.user_message())
}

/// 失敗対象のみ再試行
#[tauri::command]
#[allow(non_snake_case)]
pub async fn retry_failed_targets(
    app: AppHandle,
    request: RetryFailedRequest,
) -> Result<BulkCollectionStarted, String> {
    if bulk_collection_repository::has_running_run().map_err(|e| e.user_message())? {
        return Err("一括収集が既に実行中です".to_string());
    }

    let failed_targets = bulk_collection_repository::get_failed_targets(&request.run_id)
        .map_err(|e| e.user_message())?;
    if failed_targets.is_empty() {
        return Err("失敗した対象が見つかりません".to_string());
    }

    let run_id = bulk_collection_repository::start_run(failed_targets.len())
        .map_err(|e| e.user_message())?;
    bulk_collection_repository::register_targets(&run_id, &failed_targets)
        .map_err(|e| e.user_message())?;

    CANCEL_FLAG.store(false, Ordering::SeqCst);

    let total_targets = failed_targets.len() as i64;
    let app_clone = app.clone();
    let context = BulkCollectionStartContext {
        run_id: run_id.clone(),
        targets: failed_targets,
        total_targets,
        completed_count: 0,
        success_count: 0,
        failed_count: 0,
    };

    tokio::spawn(async move {
        process_bulk_collection(app_clone, context).await;
    });

    Ok(BulkCollectionStarted {
        run_id,
        total_targets,
    })
}

pub(crate) struct BulkCollectionStartContext {
    pub(crate) run_id: String,
    pub(crate) targets: Vec<(i64, String)>,
    pub(crate) total_targets: i64,
    pub(crate) completed_count: i64,
    pub(crate) success_count: i64,
    pub(crate) failed_count: i64,
}

fn prepare_bulk_collection_start() -> AppResult<BulkCollectionStartContext> {
    let mut conn =
        DatabaseConnection::create_connection().map_err(|e| AppError::Storage(e.to_string()))?;
    prepare_bulk_collection_start_with_connection(&mut conn)
}

pub(crate) fn prepare_bulk_collection_start_with_connection(
    conn: &mut rusqlite::Connection,
) -> AppResult<BulkCollectionStartContext> {
    if bulk_collection_repository::has_running_run_with_connection(conn)? {
        return Err(AppError::Validation("一括収集が既に実行中です".to_string()));
    }

    if let Some(run_id) = get_latest_resumable_run_with_connection(conn)? {
        let targets = get_targets_by_status_with_connection(conn, &run_id, "pending")?;
        if targets.is_empty() {
            return Err(AppError::Validation(
                "再開可能な対象がありません".to_string(),
            ));
        }

        let status = get_status_with_connection(conn, &run_id, false)?;
        resume_run_with_connection(conn, &run_id)?;

        return Ok(BulkCollectionStartContext {
            run_id,
            targets,
            total_targets: status.total_targets,
            completed_count: status.completed_count,
            success_count: status.success_count,
            failed_count: status.failed_count,
        });
    }

    let targets = get_collection_targets_with_connection(conn)?;
    if targets.is_empty() {
        return Err(AppError::Validation("収集対象が見つかりません".to_string()));
    }

    let total_targets = targets.len() as i64;
    let run_id = start_run_with_connection(conn, targets.len())?;
    register_targets_with_connection(conn, &run_id, &targets)?;

    Ok(BulkCollectionStartContext {
        run_id,
        targets,
        total_targets,
        completed_count: 0,
        success_count: 0,
        failed_count: 0,
    })
}

async fn process_bulk_collection(app: AppHandle, context: BulkCollectionStartContext) {
    info!(
        run_id = %context.run_id,
        total_targets = context.total_targets,
        "一括コミット収集を開始"
    );

    let mut completed = context.completed_count;
    let mut success_count = context.success_count;
    let mut failed_count = context.failed_count;

    for (project_id, branch_name) in context.targets {
        if CANCEL_FLAG.load(Ordering::SeqCst) {
            if let Err(err) = bulk_collection_repository::cancel_run(&context.run_id) {
                warn!("キャンセル状態の更新に失敗: {}", err);
            }
            return;
        }

        // チェックポイント取得（最後のコミット時刻）
        let since_utc = CommitRepository::get_last_commit_time(project_id, &branch_name)
            .ok()
            .flatten();

        let request = CollectCommitsRequest {
            project_id,
            branch_name: branch_name.clone(),
            since_utc,
            until_utc: None,
        };

        match collect_commits_inner(request).await {
            Ok(result) => {
                if let Err(err) = bulk_collection_repository::record_target_result(
                    &context.run_id,
                    project_id,
                    &branch_name,
                    true,
                    Some(result.inserted_count),
                    None,
                ) {
                    warn!("成功結果の記録に失敗: {}", err);
                }
                success_count += 1;
            }
            Err(err) => {
                if let Err(record_err) = bulk_collection_repository::record_target_result(
                    &context.run_id,
                    project_id,
                    &branch_name,
                    false,
                    None,
                    Some(&err.user_message()),
                ) {
                    warn!("失敗結果の記録に失敗: {}", record_err);
                }
                failed_count += 1;
            }
        }

        completed += 1;

        let payload = BulkCollectionProgress {
            run_id: context.run_id.clone(),
            total_targets: context.total_targets,
            completed_count: completed,
            success_count,
            failed_count,
            current_target: Some(TargetInfo {
                project_id,
                branch_name: branch_name.clone(),
            }),
        };

        if let Err(err) = app.emit("bulk-collection-progress", payload) {
            warn!("進捗イベント送信に失敗: {}", err);
        }

        // NOTE: GitLab API の rate limit 対策として100ms待機する
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    let completed_ok = match bulk_collection_repository::complete_run(&context.run_id) {
        Ok(()) => true,
        Err(err) => {
            warn!("完了状態の更新に失敗: {}", err);
            false
        }
    };

    if completed_ok {
        let payload = BulkCollectionProgress {
            run_id: context.run_id.clone(),
            total_targets: context.total_targets,
            completed_count: completed,
            success_count,
            failed_count,
            current_target: None,
        };

        if let Err(err) = app.emit("bulk-collection-progress", payload) {
            warn!("完了通知の送信に失敗: {}", err);
        }
    }
}
