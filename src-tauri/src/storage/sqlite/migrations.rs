/// SQLite マイグレーション管理
///
/// schema_migrations テーブルを使用してスキーマバージョンを管理し、
/// 段階的なマイグレーションを実行します。
use anyhow::{Context, Result};
use rusqlite::Connection;

/// マイグレーション実行
pub fn run_migrations(conn: &Connection) -> Result<()> {
    // schema_migrations テーブルが存在しない場合は作成
    create_schema_migrations_table(conn)?;

    // 既に適用されたマイグレーション版を取得
    let applied_versions = get_applied_versions(conn)?;
    if !applied_versions.contains(&1) {
        apply_baseline_schema(conn)?;
        record_migration(conn, 1)?;
    }

    ensure_access_token_column(conn)?;
    ensure_indexes(conn)?;

    tracing::info!("Migrations completed successfully");
    Ok(())
}

/// schema_migrations テーブルを作成
fn create_schema_migrations_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_migrations (
            version INTEGER PRIMARY KEY,
            applied_at_utc TEXT NOT NULL
        )",
        [],
    )
    .context("Failed to create schema_migrations table")?;

    Ok(())
}

/// 既に適用されたマイグレーション版一覧を取得
fn get_applied_versions(conn: &Connection) -> Result<Vec<i32>> {
    let mut stmt = conn
        .prepare("SELECT version FROM schema_migrations ORDER BY version")
        .context("Failed to prepare migration query")?;

    let versions = stmt
        .query_map([], |row| row.get::<_, i32>(0))
        .context("Failed to query applied versions")?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to collect applied versions")?;

    Ok(versions)
}

/// 単一のマイグレーションを適用
fn apply_baseline_schema(conn: &Connection) -> Result<()> {
    // NOTE: execute_batch を使用し、単一 SQL ファイル内の複数ステートメントに対応
    conn.execute_batch(include_str!("migrations/001_init.sql"))
        .context("Failed to execute baseline schema")?;
    Ok(())
}

fn record_migration(conn: &Connection, version: i32) -> Result<()> {
    // マイグレーション実行記録を保存
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO schema_migrations (version, applied_at_utc) VALUES (?, ?)",
        rusqlite::params![version, now],
    )
    .context(format!("Failed to record migration {} execution", version))?;
    Ok(())
}

fn ensure_access_token_column(conn: &Connection) -> Result<()> {
    let mut stmt = conn
        .prepare("PRAGMA table_info(connections)")
        .context("Failed to query connections table info")?;

    let has_access_token = stmt
        .query_map([], |row| row.get::<_, String>(1))
        .context("Failed to read connections columns")?
        .any(|name| name.map(|col| col == "access_token").unwrap_or(false));

    if !has_access_token {
        conn.execute("ALTER TABLE connections ADD COLUMN access_token TEXT", [])
            .context("Failed to add access_token column")?;
        conn.execute(
            "UPDATE connections SET access_token = '' WHERE access_token IS NULL",
            [],
        )
        .context("Failed to backfill access_token column")?;
    }

    Ok(())
}

fn ensure_indexes(conn: &Connection) -> Result<()> {
    let has_index: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='index' AND name='idx_commits_project_id')",
            [],
            |row| row.get(0),
        )
        .context("Failed to check baseline indexes")?;

    if !has_index {
        apply_baseline_schema(conn)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_schema_migrations_table_creation() {
        let dir = tempdir().ok();
        let db_path = dir.as_ref().unwrap().path().join("test.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();

        let result = create_schema_migrations_table(&conn);
        assert!(result.is_ok());

        // テーブルが存在することを확인
        let exists: bool = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='schema_migrations')",
            [],
            |row| row.get(0),
        ).unwrap();

        assert!(exists);
    }
}
