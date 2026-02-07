//! Tauri コマンドモジュール
//!
//! フロントエンドから呼び出される Tauri command をまとめるモジュール。
//! 各サブモジュールは OpenAPI 契約 (`contracts/tauri-commands.openapi.yaml`) に対応する。

// US1: GitLab 接続設定
pub mod gitlab_connection_get;
pub mod gitlab_connection_set;
pub mod projects_list;
pub mod projects_sync;

// US2: コミット収集
pub mod branches_list;
pub mod commits_collect;
pub mod commits_collect_bulk;

// US3: 月次集計
pub mod stats_monthly_cross_view;
pub mod stats_monthly_project_view;

// ブランチ削除
pub mod commits_branch_delete;
pub mod commits_branch_delete_impact;

// ユーザーフィルタ
pub mod user_filter_get;
pub mod user_filter_set;

#[cfg(test)]
mod commits_collect_bulk_test;

// Re-export for convenience
pub use branches_list::*;
pub use commits_branch_delete::*;
pub use commits_branch_delete_impact::*;
pub use commits_collect::*;
pub use commits_collect_bulk::*;
pub use gitlab_connection_get::*;
pub use gitlab_connection_set::*;
pub use projects_list::*;
pub use projects_sync::*;
pub use stats_monthly_cross_view::*;
pub use stats_monthly_project_view::*;
pub use user_filter_get::*;
pub use user_filter_set::*;
