# Research: 一括コミット収集の継続

**Date**: 2026-02-06  
**Feature**: 001-bulk-commit-collect

## Research Questions

### 1. 既存の単一コミット収集ロジックの再利用方法

**Question**: `commits_collect` コマンドのロジックを一括処理でどのように再利用するか？

**Findings**:

- **既存実装の確認**: `src-tauri/src/commands/commits_collect.rs` に単一対象（project_id + branch_name）の収集ロジックが存在する。
- **再利用アプローチ**:
  - 内部関数 `collect_commits_inner(project_id, branch_name, since_utc)` を抽出し、一括処理ループから呼び出す。
  - 各対象の処理結果（成功/失敗、新規コミット件数、エラーメッセージ）を構造体で返す。
- **利点**: 既存のGitLab API呼び出し、エラーハンドリング、データ保存ロジックをそのまま活用できる。

**Decision**: `collect_commits_inner` 関数を pub(crate) で公開し、`commits_collect_bulk` から呼び出す。

**Rationale**: コードの重複を避け、テスト済みの既存ロジックを活用することで、実装の信頼性を高める。

**Alternatives Considered**:

- ロジックを完全に複製 → メンテナンスコストが高く、バグ修正時に両方を更新する必要がある。
- 単一コマンドに一括処理モードを追加 → 責務が混在し、コマンドの複雑度が上がる。

---

### 2. 一括処理の状態管理とエラーハンドリングのベストプラクティス

**Question**: Rust/Tauri で複数対象を順次処理する際の状態管理とエラーハンドリングのパターンは？

**Findings**:

- **Tauri のベストプラクティス**:
  - 長時間実行コマンドは `async` 関数として実装し、tokio ランタイムで実行する。
  - 各対象の処理結果を `Vec<CollectionResult>` に蓄積し、最後に一括で返す。
  - 個別のエラーは `Result<T, E>` で捕捉し、全体の処理は継続する（fail-fast ではない）。
- **状態の永続化**:
  - 各対象の処理完了後に SQLite に結果を保存（`bulk_collection_runs` テーブル）。
  - 中断時には、未処理の対象を識別できるよう、処理状態を記録する。

**Decision**:

- 各対象の処理結果を即座に SQLite に保存する（トランザクション単位）。
- エラーはキャプチャしてログに記録し、`CollectionResult::Failed` として結果リストに追加。
- 全対象の処理後に結果サマリを返す。

**Rationale**: 中断時のデータ損失を防ぎ、再開時に重複処理を避けるため。

**Alternatives Considered**:

- メモリ内で結果を保持し、最後に一括保存 → 中断時にデータが失われる。
- fail-fast（最初のエラーで中止） → FR-003（失敗対象のみ再試行）の要件を満たせない。

---

### 3. 進捗表示とキャンセル処理のパターン

**Question**: Tauri + React で長時間実行コマンドの進捗を表示し、ユーザーがキャンセルする方法は？

**Findings**:

- **進捗表示のパターン**:
  - Tauri の `emit` API を使って、バックエンドからフロントエンドにイベントを送信する。
  - React側で `listen` API を使ってイベントを受信し、状態を更新する。
  - 進捗イベント: `{ total, completed, failed, current_target }`
- **キャンセル処理**:
  - `tokio::sync::oneshot` チャネルを使って、フロントエンドからのキャンセル要求を受け取る。
  - または、`Arc<AtomicBool>` をキャンセルフラグとして使い、各イテレーションでチェックする。
  - 実装の簡便性から、後者（AtomicBool）を推奨。

**Decision**:

- `tauri::emit` で進捗イベントを各対象の処理後に送信。
- `Arc<AtomicBool>` をキャンセルフラグとして使用し、専用コマンド `cancel_bulk_collection` で設定。

**Rationale**: Tauri の標準的なパターンで、実装がシンプルで追加の依存関係が不要。

**Alternatives Considered**:

- WebSocket/SSE → Tauri では不要な複雑性。
- ポーリング → リアルタイム性に欠ける。

---

### 4. SQLite での部分結果の保存とトランザクション管理

**Question**: 各対象の処理後に結果を保存する際のトランザクション戦略は？

**Findings**:

- **トランザクション単位**:
  - 各対象の処理を1つのトランザクションとする（コミット保存 + 結果記録）。
  - これにより、部分的な成功/失敗が正確に記録される。
- **スキーマ設計**:

  ```sql
  CREATE TABLE bulk_collection_runs (
    run_id TEXT PRIMARY KEY,
    started_at_utc TEXT NOT NULL,
    status TEXT NOT NULL, -- 'running' | 'completed' | 'cancelled'
    total_targets INTEGER NOT NULL,
    completed_count INTEGER NOT NULL,
    failed_count INTEGER NOT NULL
  );

  CREATE TABLE bulk_collection_results (
    run_id TEXT NOT NULL,
    project_id INTEGER NOT NULL,
    branch_name TEXT NOT NULL,
    status TEXT NOT NULL, -- 'success' | 'failed' | 'pending'
    new_commits_count INTEGER,
    error_message TEXT,
    processed_at_utc TEXT,
    FOREIGN KEY (run_id) REFERENCES bulk_collection_runs(run_id)
  );
  ```

**Decision**:

- 各対象の処理を独立したトランザクションで実行。
- 結果を `bulk_collection_results` テーブルに即座に記録。
- 実行全体の状態を `bulk_collection_runs` テーブルで管理。

**Rationale**: 部分的な進捗を保存し、中断後の再開を可能にする。

**Alternatives Considered**:

- 全体を1つのトランザクションで実行 → 中断時にロールバックされ、進捗が失われる。
- トランザクションなし → データの整合性が保証されない。

---

### 5. Tauri コマンドの長時間実行とタイムアウト処理

**Question**: 100件の対象を10分で処理する場合、Tauri コマンドのタイムアウトやリソース管理の考慮事項は？

**Findings**:

- **Tauri のタイムアウト**:
  - デフォルトでは Tauri コマンドにタイムアウトは設定されていない。
  - 長時間実行コマンドは問題なく動作する。
- **リソース管理**:
  - GitLab API の rate limit に注意（通常は 600 req/min）。
  - 順次実行により、同時接続数を1に制限し、rate limit を回避しやすくする。
  - 各対象の処理後に短い待機時間（例: 100ms）を入れることで、さらに余裕を持たせる。
- **メモリ管理**:
  - 結果を逐次 SQLite に保存することで、メモリ使用量を一定に保つ。
  - `Vec<CollectionResult>` は最終結果のサマリのみを保持（詳細は DB）。

**Decision**:

- Tauri コマンドのタイムアウト設定は不要。
- 各対象の処理間に 100ms の待機時間を追加（rate limit 配慮）。
- 結果の詳細は SQLite に保存し、コマンドの戻り値はサマリのみとする。

**Rationale**: システムリソースと外部 API の制約を適切に管理し、安定した動作を確保する。

**Alternatives Considered**:

- 明示的なタイムアウト設定 → 100件の処理時間は可変なため、固定タイムアウトは不適切。
- 並行実行 → rate limit に抵触するリスクが高い。

---

## Technology Stack Validation

**Chosen Technologies**:

- **Backend**: Rust + Tauri 2 + rusqlite
- **Frontend**: React 18.3 + TypeScript + shadcn/ui
- **State Management**: React hooks (useState, useEffect)
- **Event Communication**: Tauri event system (emit/listen)

**Validation**:

- 既存プロジェクトとの整合性が高く、追加の依存関係は不要。
- Tauri 2 の event system は長時間実行タスクの進捗表示に適している。
- SQLite は軽量で、部分結果の保存に最適。

---

## Summary of Decisions

| 領域             | 決定事項                                 | 根拠                             |
| ---------------- | ---------------------------------------- | -------------------------------- |
| ロジック再利用   | `collect_commits_inner` を公開して再利用 | コード重複回避、既存ロジック活用 |
| 状態管理         | SQLite に各対象の結果を即座に保存        | 中断時の進捗保持、再開可能性     |
| 進捗表示         | Tauri emit/listen でイベント通信         | 標準的なパターン、実装が簡潔     |
| キャンセル       | Arc<AtomicBool> フラグ                   | シンプルで追加依存なし           |
| トランザクション | 各対象を独立したトランザクションで処理   | 部分成功の記録、整合性確保       |
| Rate Limit 対策  | 順次実行 + 100ms 待機                    | API制限回避、安定性向上          |

## Security Note

- 一括収集の `error_message` には accessToken/authorEmail を含めない。記録前に秘匿情報を除外し、ユーザー向けの安全なメッセージだけを保存する。

---

## Next Steps

Phase 1 で以下を生成：

- **data-model.md**: `bulk_collection_runs` と `bulk_collection_results` テーブルの詳細スキーマ
- **contracts/**: `commits_collect_bulk` と `cancel_bulk_collection` の Tauri コマンド API 仕様
- **quickstart.md**: 実装の概要、テスト戦略、デプロイ手順
