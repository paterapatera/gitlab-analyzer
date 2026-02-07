//! GitLab 月次コミット行数分析アプリケーション
//!
//! GitLab からコミットを取得し、年/月/ユーザー単位で集計・可視化する Tauri アプリ。

pub mod commands;
pub mod domain;
pub mod error;
pub mod gitlab;
pub mod logging;
pub mod paths;
pub mod stats;
pub mod storage;

use commands::{
    cancel_bulk_collection, collect_commits, collect_commits_bulk, delete_branch_commits,
    get_branch_delete_impact, get_bulk_collection_status, get_gitlab_connection,
    get_monthly_stats_cross_view, get_monthly_stats_project_view, get_projects, list_branches,
    retry_failed_targets, set_gitlab_connection, sync_projects, user_filter_get, user_filter_set,
};

/// Tauri アプリケーションのエントリーポイント
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // ログ初期化
    logging::init_logging();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|_app| {
            // SQLite データベース初期化
            initialize_sqlite()?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // US1: 接続設定
            get_gitlab_connection,
            set_gitlab_connection,
            get_projects,
            sync_projects,
            // US2: コミット収集
            list_branches,
            collect_commits,
            collect_commits_bulk,
            cancel_bulk_collection,
            get_bulk_collection_status,
            retry_failed_targets,
            // US3: 月次集計
            get_monthly_stats_project_view,
            get_monthly_stats_cross_view,
            // ブランチ削除
            get_branch_delete_impact,
            delete_branch_commits,
            // ユーザーフィルタ
            user_filter_get,
            user_filter_set,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
/// SQLite データベース初期化
///
/// アプリケーション起動時にデータベース接続を確立し、
/// マイグレーションを実行します。
fn initialize_sqlite() -> Result<(), Box<dyn std::error::Error>> {
    let conn = storage::sqlite::DatabaseConnection::create_connection()?;
    storage::sqlite::run_migrations(&conn)?;
    tracing::info!("SQLite database initialized successfully");
    Ok(())
}
