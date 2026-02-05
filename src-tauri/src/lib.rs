//! GitLab 月次コミット行数分析アプリケーション
//!
//! GitLab からコミットを取得し、年/月/ユーザー単位で集計・可視化する Tauri アプリ。

pub mod error;
pub mod logging;
pub mod paths;
pub mod domain;
pub mod storage;
pub mod gitlab;
pub mod stats;
pub mod commands;

use commands::{
    get_gitlab_connection, set_gitlab_connection, get_projects, sync_projects,
    list_branches, collect_commits,
    get_monthly_stats_project_view, get_monthly_stats_cross_view,
    user_filter_get, user_filter_set,
};

/// Tauri アプリケーションのエントリーポイント
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // ログ初期化
    logging::init_logging();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // US1: 接続設定
            get_gitlab_connection,
            set_gitlab_connection,
            get_projects,
            sync_projects,
            // US2: コミット収集
            list_branches,
            collect_commits,
            // US3: 月次集計
            get_monthly_stats_project_view,
            get_monthly_stats_cross_view,
            // ユーザーフィルタ
            user_filter_get,
            user_filter_set,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
