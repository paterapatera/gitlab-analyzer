# Data Model: 一括コミット収集の継続

## Entities

### bulk_collection_runs（新規）

- Purpose: 一括収集の実行単位を記録し、実行状態と進捗を管理する
- Fields:
  - run_id: text (PK, UUID v4 形式)
  - started_at_utc: text (ISO-8601, not null)
  - completed_at_utc: text (ISO-8601, nullable)
  - status: text (not null, enum: 'running' | 'completed' | 'cancelled')
  - total_targets: integer (not null, 対象数)
  - completed_count: integer (not null, default 0)
  - failed_count: integer (not null, default 0)
  - success_count: integer (not null, default 0)
- Constraints:
  - status は 'running', 'completed', 'cancelled' のいずれか
  - completed_count + failed_count <= total_targets

### bulk_collection_results（新規）

- Purpose: 一括収集における各対象の処理結果を記録する
- Fields:
  - run_id: text (FK -> bulk_collection_runs.run_id, not null)
  - project_id: integer (not null)
  - branch_name: text (not null)
  - status: text (not null, enum: 'pending' | 'success' | 'failed')
  - new_commits_count: integer (nullable, 成功時のみ)
  - error_message: text (nullable, 失敗時のエラー内容)
  - processed_at_utc: text (ISO-8601, nullable, 処理完了時刻)
- Constraints:
  - Unique: (run_id, project_id, branch_name)
  - status は 'pending', 'success', 'failed' のいずれか
  - status = 'success' の場合、new_commits_count は not null
  - status = 'failed' の場合、error_message は not null

### commits（既存、変更なし）

- Purpose: コミット情報の永続化（既存テーブル）
- Fields:
  - project_id: integer (FK -> projects.project_id)
  - branch_name: text (not null)
  - sha: text (not null)
  - author_name: text (not null)
  - author_email: text (not null)
  - committed_date_utc: text (ISO-8601, not null)
  - additions: integer (not null)
  - deletions: integer (not null)
- Constraints:
  - Unique: (project_id, branch_name, sha)
- **Note**: committed_date_utc の最大値がチェックポイントとして機能する

## Relationships

- bulk_collection_runs 1 --- \* bulk_collection_results
- commits テーブルとは直接の FK 制約はないが、論理的に関連（bulk_collection_results の各レコードが commits テーブルに新規レコードを追加する）

## Indexes

### 新規追加

- bulk_collection_results(run_id, status)
- bulk_collection_results(project_id, branch_name, status)
- bulk_collection_runs(status, started_at_utc)

### 既存（活用）

- commits(project_id, branch_name, committed_date_utc) ← チェックポイント取得に使用

## Validation Rules

- run_id は UUID v4 形式（例: "550e8400-e29b-41d4-a716-446655440000"）
- status == 'pending' の結果レコードは、処理中または未処理を示す
- status == 'running' の実行レコードは、同時に1つまで存在する（二重実行防止）
- completed_at_utc は status が 'completed' または 'cancelled' の場合のみ設定される

## State Transitions

### bulk_collection_runs.status

1. **running** → 一括収集開始時に設定
2. **running → completed** → 全対象の処理完了時
3. **running → cancelled** → ユーザーがキャンセルした場合

### bulk_collection_results.status

1. **pending** → 処理対象として登録時
2. **pending → success** → 対象の収集が成功した場合
3. **pending → failed** → 対象の収集が失敗した場合

## Query Patterns

### 一括収集の対象取得

```sql
-- 収集履歴がある (project_id, branch_name) ペアを取得
SELECT DISTINCT project_id, branch_name
FROM commits
ORDER BY project_id, branch_name;
```

### チェックポイント取得

```sql
-- 特定の (project_id, branch_name) の最後の収集時刻を取得
SELECT MAX(committed_date_utc) as last_collect_time
FROM commits
WHERE project_id = ? AND branch_name = ?;
```

### 実行中の一括収集の確認

```sql
-- 実行中の一括収集があるかチェック（二重実行防止）
SELECT COUNT(*) FROM bulk_collection_runs
WHERE status = 'running';
```

### 失敗対象の再試行リスト取得

```sql
-- 特定の run_id で失敗した対象を取得
SELECT project_id, branch_name
FROM bulk_collection_results
WHERE run_id = ? AND status = 'failed';
```

### 結果サマリ取得

```sql
-- 一括収集の結果サマリ
SELECT
  r.run_id,
  r.started_at_utc,
  r.completed_at_utc,
  r.status as run_status,
  r.total_targets,
  r.success_count,
  r.failed_count,
  COUNT(CASE WHEN br.status = 'pending' THEN 1 END) as pending_count
FROM bulk_collection_runs r
LEFT JOIN bulk_collection_results br ON r.run_id = br.run_id
WHERE r.run_id = ?
GROUP BY r.run_id;
```

## Migration Strategy

### V1: 初期スキーマ作成

```sql
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

CREATE INDEX IF NOT EXISTS idx_bulk_results_run_status
  ON bulk_collection_results(run_id, status);

CREATE INDEX IF NOT EXISTS idx_bulk_results_target_status
  ON bulk_collection_results(project_id, branch_name, status);

CREATE INDEX IF NOT EXISTS idx_bulk_runs_status_started
  ON bulk_collection_runs(status, started_at_utc);
```

## Data Lifecycle

- **bulk_collection_runs**: 永続保存（ユーザーが履歴として参照可能）
- **bulk_collection_results**: 永続保存（失敗対象の再試行に必要）
- **古いレコードの削除**: 将来的に保持期間（例: 30日）を設定して自動削除する機能を追加可能
