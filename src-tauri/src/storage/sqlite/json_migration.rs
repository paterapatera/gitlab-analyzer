/// JSON ストア → SQLite マイグレーション
///
/// 既存の JSON ファイルから SQLite へのデータ移行を実行します。
use anyhow::{Context, Result};
use rusqlite::Connection;
use std::path::Path;

pub struct JsonMigration;

impl JsonMigration {
    /// JSON ストアから SQLite への移行を実行
    pub fn migrate_json_to_sqlite(
        sqlite_conn: &mut Connection,
        json_store_path: &Path,
    ) -> Result<MigrationStats> {
        let mut stats = MigrationStats::default();

        // JSON ファイルの存在確認
        if !json_store_path.exists() {
            tracing::info!(
                "No JSON store found at {:?}, skipping migration",
                json_store_path
            );
            return Ok(stats);
        }

        // JSON ファイルを読み込む
        let json_content =
            std::fs::read_to_string(json_store_path).context("Failed to read JSON store file")?;

        let store: serde_json::Value =
            serde_json::from_str(&json_content).context("Failed to parse JSON store")?;

        // トランザクション開始
        let tx = sqlite_conn
            .transaction()
            .context("Failed to start migration transaction")?;

        // 接続設定の移行（存在する場合）
        if let Some(connection) = store.get("connection") {
            if let Err(e) = Self::migrate_connection(&tx, connection) {
                tracing::warn!("Failed to migrate connection: {}", e);
            } else {
                stats.connections_migrated = 1;
            }
        }

        // プロジェクト情報の移行
        if let Some(projects) = store.get("projects").and_then(|v| v.as_array()) {
            for project in projects {
                if let Err(e) = Self::migrate_project(&tx, project) {
                    tracing::warn!("Failed to migrate project: {}", e);
                } else {
                    stats.projects_migrated += 1;
                }
            }
        }

        // コミット情報の移行
        if let Some(commits) = store.get("commits").and_then(|v| v.as_array()) {
            for commit in commits {
                if let Err(e) = Self::migrate_commit(&tx, commit) {
                    tracing::warn!("Failed to migrate commit: {}", e);
                } else {
                    stats.commits_migrated += 1;
                }
            }
        }

        // ユーザーフィルタの移行
        if let Some(user_filters) = store.get("user_filters").and_then(|v| v.as_array()) {
            for filter in user_filters {
                if let Err(e) = Self::migrate_user_filter(&tx, filter) {
                    tracing::warn!("Failed to migrate user filter: {}", e);
                } else {
                    stats.user_filters_migrated += 1;
                }
            }
        }

        // コミット
        tx.commit().context("Failed to commit migration")?;

        tracing::info!(
            "JSON migration completed: {} connections, {} projects, {} commits, {} user filters",
            stats.connections_migrated,
            stats.projects_migrated,
            stats.commits_migrated,
            stats.user_filters_migrated,
        );

        Ok(stats)
    }

    fn migrate_connection(conn: &Connection, data: &serde_json::Value) -> Result<()> {
        let base_url = data
            .get("base_url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing base_url"))?;

        let author_email = data.get("author_email").and_then(|v| v.as_str());
        let access_token = data
            .get("access_token")
            .or_else(|| data.get("accessToken"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let updated_at = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT OR REPLACE INTO connections (base_url, author_email, access_token, updated_at_utc)
             VALUES (?, ?, ?, ?)",
            rusqlite::params![base_url, author_email, access_token, updated_at],
        ).context("Failed to migrate connection")?;

        Ok(())
    }

    fn migrate_project(conn: &Connection, data: &serde_json::Value) -> Result<()> {
        let project_id = data
            .get("project_id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| anyhow::anyhow!("Missing project_id"))?;

        let name = data
            .get("name")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing name"))?;

        let path_with_namespace = data
            .get("path_with_namespace")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let web_url = data.get("web_url").and_then(|v| v.as_str()).unwrap_or("");

        let last_sync_time = data.get("last_sync_time_utc").and_then(|v| v.as_str());

        conn.execute(
            "INSERT OR REPLACE INTO projects (project_id, name, path_with_namespace, web_url, last_sync_time_utc)
             VALUES (?, ?, ?, ?, ?)",
            rusqlite::params![project_id, name, path_with_namespace, web_url, last_sync_time],
        ).context("Failed to migrate project")?;

        Ok(())
    }

    fn migrate_commit(conn: &Connection, data: &serde_json::Value) -> Result<()> {
        let project_id = data
            .get("project_id")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| anyhow::anyhow!("Missing project_id"))? as i32;

        let branch_name = data
            .get("branch_name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let sha = data
            .get("sha")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing sha"))?;

        let author_name = data
            .get("author_name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let author_email = data
            .get("author_email")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        let committed_date_utc = data
            .get("committed_date_utc")
            .and_then(|v| v.as_str())
            .unwrap_or("1970-01-01T00:00:00Z");

        let additions = data.get("additions").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

        let deletions = data.get("deletions").and_then(|v| v.as_i64()).unwrap_or(0) as i32;

        conn.execute(
            "INSERT OR REPLACE INTO commits
             (project_id, branch_name, sha, author_name, author_email, committed_date_utc, additions, deletions)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            rusqlite::params![
                project_id, branch_name, sha, author_name, author_email, committed_date_utc, additions, deletions
            ],
        ).context("Failed to migrate commit")?;

        Ok(())
    }

    fn migrate_user_filter(conn: &Connection, data: &serde_json::Value) -> Result<()> {
        let view_type = data
            .get("view_type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing view_type"))?;

        let context_key = data
            .get("context_key")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        let selected_users_json = data
            .get("selected_users_json")
            .map(|v| v.to_string())
            .unwrap_or_else(|| "[]".to_string());

        let updated_at = chrono::Utc::now().to_rfc3339();

        conn.execute(
            "INSERT OR REPLACE INTO user_filters (view_type, context_key, selected_users_json, updated_at_utc)
             VALUES (?, ?, ?, ?)",
            rusqlite::params![view_type, context_key, selected_users_json, updated_at],
        ).context("Failed to migrate user filter")?;

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct MigrationStats {
    pub connections_migrated: usize,
    pub projects_migrated: usize,
    pub commits_migrated: usize,
    pub user_filters_migrated: usize,
}

pub fn migrate_json_to_sqlite(
    sqlite_conn: &mut Connection,
    json_store_path: &Path,
) -> Result<MigrationStats> {
    JsonMigration::migrate_json_to_sqlite(sqlite_conn, json_store_path)
}
