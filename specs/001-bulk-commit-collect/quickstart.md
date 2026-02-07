# Quickstart: 一括コミット収集の継続

**Feature**: 001-bulk-commit-collect  
**Date**: 2026-02-06

## 概要

収集履歴がある全ての (project_id, branch_name) ペアについて、ワンボタンで続きを一括収集できる機能。進捗表示と失敗対象の再試行をサポートし、中断時にも部分結果を保持する。

## アーキテクチャ

```
┌─────────────────────────────────────┐
│  UI: BulkCollectCard.tsx            │
│  - ボタン（開始/キャンセル/再試行） │
│  - 進捗表示（総数/完了/失敗）       │
│  - 結果サマリ（対象ごとの結果）     │
└──────────────┬──────────────────────┘
               │ Tauri invoke
               ▼
┌─────────────────────────────────────┐
│  Command: commits_collect_bulk      │
│  - 対象リスト取得                   │
│  - 各対象を順次処理                 │
│  - 進捗イベント emit                │
│  - 結果を SQLite に保存             │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│  Repository:                        │
│  - bulk_collection_repository.rs    │
│  - commit_repository.rs（再利用）   │
└──────────────┬──────────────────────┘
               │
               ▼
┌─────────────────────────────────────┐
│  Storage: SQLite                    │
│  - bulk_collection_runs             │
│  - bulk_collection_results          │
│  - commits（既存）                  │
└─────────────────────────────────────┘
```

## 実装の流れ

### Phase 1: バックエンド（Rust/Tauri）

#### 1.1 データモデルとマイグレーション

**ファイル**: `src-tauri/src/storage/sqlite/migrations.rs`

```rust
// V6: 一括収集テーブルの追加
pub fn migrate_v6(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS bulk_collection_runs (
          run_id TEXT PRIMARY KEY,
          started_at_utc TEXT NOT NULL,
          completed_at_utc TEXT,
          status TEXT NOT NULL CHECK(status IN ('running', 'completed', 'cancelled')),
          total_targets INTEGER NOT NULL,
          completed_count INTEGER NOT NULL DEFAULT 0,
          failed_count INTEGER NOT NULL DEFAULT 0,
          success_count INTEGER NOT NULL DEFAULT 0
        );

        CREATE TABLE IF NOT EXISTS bulk_collection_results (
          run_id TEXT NOT NULL,
          project_id INTEGER NOT NULL,
          branch_name TEXT NOT NULL,
          status TEXT NOT NULL CHECK(status IN ('pending', 'success', 'failed')),
          new_commits_count INTEGER,
          error_message TEXT,
          processed_at_utc TEXT,
          PRIMARY KEY (run_id, project_id, branch_name),
          FOREIGN KEY (run_id) REFERENCES bulk_collection_runs(run_id) ON DELETE CASCADE
        );

        CREATE INDEX idx_bulk_results_run_status
          ON bulk_collection_results(run_id, status);
        CREATE INDEX idx_bulk_results_target_status
          ON bulk_collection_results(project_id, branch_name, status);
        CREATE INDEX idx_bulk_runs_status_started
          ON bulk_collection_runs(status, started_at_utc);
        "#
    )?;
    Ok(())
}
```

#### 1.2 リポジトリレイヤー

**ファイル**: `src-tauri/src/storage/bulk_collection_repository.rs`

```rust
use anyhow::Result;
use rusqlite::{Connection, params};
use uuid::Uuid;

/// 一括収集の実行を開始
pub fn start_bulk_collection_run(
    conn: &Connection,
    total_targets: usize,
) -> Result<String> {
    let run_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    conn.execute(
        "INSERT INTO bulk_collection_runs (run_id, started_at_utc, status, total_targets)
         VALUES (?1, ?2, 'running', ?3)",
        params![run_id, now, total_targets as i64],
    )?;

    Ok(run_id)
}

/// 対象を pending 状態で登録
pub fn register_targets(
    conn: &Connection,
    run_id: &str,
    targets: &[(i64, String)],  // (project_id, branch_name)
) -> Result<()> {
    let tx = conn.transaction()?;

    for (project_id, branch_name) in targets {
        tx.execute(
            "INSERT INTO bulk_collection_results (run_id, project_id, branch_name, status)
             VALUES (?1, ?2, ?3, 'pending')",
            params![run_id, project_id, branch_name],
        )?;
    }

    tx.commit()?;
    Ok(())
}

/// 対象の処理結果を記録
pub fn record_target_result(
    conn: &Connection,
    run_id: &str,
    project_id: i64,
    branch_name: &str,
    success: bool,
    new_commits_count: Option<usize>,
    error_message: Option<&str>,
) -> Result<()> {
    let now = chrono::Utc::now().to_rfc3339();
    let status = if success { "success" } else { "failed" };

    conn.execute(
        "UPDATE bulk_collection_results
         SET status = ?1, new_commits_count = ?2, error_message = ?3, processed_at_utc = ?4
         WHERE run_id = ?5 AND project_id = ?6 AND branch_name = ?7",
        params![
            status,
            new_commits_count.map(|c| c as i64),
            error_message,
            now,
            run_id,
            project_id,
            branch_name,
        ],
    )?;

    // 実行レコードのカウンターを更新
    let count_field = if success { "success_count" } else { "failed_count" };
    conn.execute(
        &format!(
            "UPDATE bulk_collection_runs
             SET completed_count = completed_count + 1, {} = {} + 1
             WHERE run_id = ?1",
            count_field, count_field
        ),
        params![run_id],
    )?;

    Ok(())
}

// その他のメソッド: complete_run, cancel_run, get_status, get_failed_targets など
```

#### 1.3 コマンドレイヤー

**ファイル**: `src-tauri/src/commands/commits_collect_bulk.rs`

```rust
use tauri::{AppHandle, Emitter};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

// グローバルなキャンセルフラグ
static CANCEL_FLAG: AtomicBool = AtomicBool::new(false);

/// 一括収集を開始
#[tauri::command]
pub async fn collect_commits_bulk(
    app: AppHandle,
) -> Result<BulkCollectionStarted, String> {
    // 二重実行チェック
    let conn = DatabaseConnection::create_connection()
        .map_err(|e| e.to_string())?;

    if is_running(&conn)? {
        return Err("一括収集が既に実行中です".to_string());
    }

    // 対象リスト取得
    let targets = get_collection_targets(&conn)?;
    if targets.is_empty() {
        return Err("収集対象が見つかりません".to_string());
    }

    // 実行開始
    let run_id = bulk_collection_repository::start_bulk_collection_run(
        &conn, targets.len()
    )?;
    bulk_collection_repository::register_targets(&conn, &run_id, &targets)?;

    // キャンセルフラグリセット
    CANCEL_FLAG.store(false, Ordering::SeqCst);

    // 非同期で処理開始
    let app_clone = app.clone();
    let run_id_clone = run_id.clone();
    tokio::spawn(async move {
        process_bulk_collection(app_clone, run_id_clone, targets).await;
    });

    Ok(BulkCollectionStarted {
        run_id,
        total_targets: targets.len(),
    })
}

/// 各対象を順次処理
async fn process_bulk_collection(
    app: AppHandle,
    run_id: String,
    targets: Vec<(i64, String)>,
) {
    let conn = match DatabaseConnection::create_connection() {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Failed to create connection: {}", e);
            return;
        }
    };

    let total = targets.len();
    let mut completed = 0;
    let mut success_count = 0;
    let mut failed_count = 0;

    for (project_id, branch_name) in targets {
        // キャンセルチェック
        if CANCEL_FLAG.load(Ordering::SeqCst) {
            bulk_collection_repository::cancel_run(&conn, &run_id).ok();
            break;
        }

        // チェックポイント取得
        let since_utc = commit_repository::get_last_commit_time(
            &conn, project_id, &branch_name
        ).ok().flatten();

        // 単一対象の収集を実行
        let result = collect_commits_inner(project_id, &branch_name, since_utc).await;

        // 結果を記録
        match result {
            Ok(count) => {
                bulk_collection_repository::record_target_result(
                    &conn, &run_id, project_id, &branch_name,
                    true, Some(count), None
                ).ok();
                success_count += 1;
            }
            Err(e) => {
                bulk_collection_repository::record_target_result(
                    &conn, &run_id, project_id, &branch_name,
                    false, None, Some(&e.to_string())
                ).ok();
                failed_count += 1;
            }
        }

        completed += 1;

        // 進捗イベント emit
        app.emit("bulk-collection-progress", BulkCollectionProgress {
            run_id: run_id.clone(),
            total_targets: total,
            completed_count: completed,
            success_count,
            failed_count,
            current_target: Some(TargetInfo { project_id, branch_name }),
        }).ok();

        // Rate limit 配慮（100ms 待機）
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }

    // 完了
    bulk_collection_repository::complete_run(&conn, &run_id).ok();
}

/// キャンセルコマンド
#[tauri::command]
pub fn cancel_bulk_collection() -> Result<(), String> {
    CANCEL_FLAG.store(true, Ordering::SeqCst);
    Ok(())
}
```

### Phase 2: フロントエンド（React/TypeScript）

#### 2.1 UI コンポーネント

**ファイル**: `src/features/collect/BulkCollectCard.tsx`

```tsx
import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card'
import { Button } from '@/components/ui/button'
import { Progress } from '@/components/ui/progress'

interface BulkCollectionProgress {
  runId: string
  totalTargets: number
  completedCount: number
  successCount: number
  failedCount: number
}

export function BulkCollectCard() {
  const [isRunning, setIsRunning] = useState(false)
  const [progress, setProgress] = useState<BulkCollectionProgress | null>(null)

  useEffect(() => {
    const unlisten = listen<BulkCollectionProgress>('bulk-collection-progress', (event) => {
      setProgress(event.payload)
      if (event.payload.completedCount === event.payload.totalTargets) {
        setIsRunning(false)
      }
    })

    return () => {
      unlisten.then((fn) => fn())
    }
  }, [])

  const handleStart = async () => {
    try {
      setIsRunning(true)
      await invoke('collect_commits_bulk')
    } catch (error) {
      console.error('Failed to start bulk collection:', error)
      setIsRunning(false)
    }
  }

  const handleCancel = async () => {
    try {
      await invoke('cancel_bulk_collection')
    } catch (error) {
      console.error('Failed to cancel:', error)
    }
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>一括収集</CardTitle>
      </CardHeader>
      <CardContent>
        <div className="space-y-4">
          <div className="flex gap-2">
            <Button onClick={handleStart} disabled={isRunning}>
              すべての続きを収集
            </Button>
            {isRunning && (
              <Button onClick={handleCancel} variant="outline">
                キャンセル
              </Button>
            )}
          </div>

          {progress && (
            <div className="space-y-2">
              <Progress value={(progress.completedCount / progress.totalTargets) * 100} />
              <div className="text-sm text-muted-foreground">
                {progress.completedCount} / {progress.totalTargets} 完了 （成功:{' '}
                {progress.successCount}, 失敗: {progress.failedCount}）
              </div>
            </div>
          )}
        </div>
      </CardContent>
    </Card>
  )
}
```

## テスト戦略

### バックエンド（Rust）

**ファイル**: `src-tauri/src/storage/bulk_collection_repository_test.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_start_and_complete_run() {
        let dir = tempdir().unwrap();
        let conn = setup_test_db(&dir);

        let run_id = start_bulk_collection_run(&conn, 5).unwrap();
        assert!(!run_id.is_empty());

        complete_run(&conn, &run_id).unwrap();

        let status = get_run_status(&conn, &run_id).unwrap();
        assert_eq!(status.status, "completed");
    }

    #[test]
    fn test_record_target_results() {
        // 成功/失敗ケースのテスト
    }
}
```

### フロントエンド（React）

**ファイル**: `src/features/collect/BulkCollectCard.test.tsx`

```tsx
import { render, screen, fireEvent } from '@testing-library/react'
import { BulkCollectCard } from './BulkCollectCard'
import { vi } from 'vitest'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

describe('BulkCollectCard', () => {
  it('renders start button', () => {
    render(<BulkCollectCard />)
    expect(screen.getByText('すべての続きを収集')).toBeInTheDocument()
  })

  it('disables button when running', async () => {
    // テストロジック
  })
})
```

## 実装チェックリスト

### Phase 1: バックエンド

- [x] マイグレーション（V6）の追加
- [x] `bulk_collection_repository.rs` の実装
- [x] `commits_collect_bulk.rs` コマンドの実装
- [x] キャンセルコマンドの実装
- [x] 単体テストの追加

### Phase 2: フロントエンド

- [x] `BulkCollectCard.tsx` の実装
- [x] 進捗イベントリスナーの実装
- [x] エラーハンドリングの追加
- [x] UI テストの追加

### Phase 3: 統合

- [x] コマンドを `lib.rs` に登録
- [ ] E2E シナリオのテスト
- [ ] パフォーマンステスト（100件の対象で10分以内）

## 動作確認

1. 収集履歴がある対象を複数準備
2. 一括収集ボタンをクリック
3. 進捗バーが更新されることを確認
4. 結果サマリが表示されることを確認
5. 失敗対象がある場合、再試行ボタンが機能することを確認
6. 途中キャンセル後に再開すると pending のみ処理されることを確認

## パフォーマンス目標

- **SC-001**: 100件の対象の90%が10分以内に完了
- **SC-002**: 一括収集開始は10秒以内、進捗表示は5秒以内

## セキュリティ考慮事項

- 結果サマリに `accessToken` や `authorEmail` を含めない
- SQLite に保存される一括収集結果にも機密情報を含めない（プロジェクトID、ブランチ名のみ）
- エラーメッセージから機密情報を除外
