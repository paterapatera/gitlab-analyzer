# Quickstart: SQLiteストレージへの移行

## Goal

JSON ファイルストレージを SQLite に置き換え、既存 Tauri コマンドの挙動を維持しながら大量データでも高速に動作するようにする。

## Implementation Steps

1. **SQLite 基盤の追加**
   - `src-tauri/src/storage/sqlite/` を新設し、接続生成（データディレクトリ + db ファイル名）と PRAGMA 設定（WAL、busy_timeout）を実装する。
   - `schema_migrations` テーブルを作成し、スキーマバージョンを管理する。

2. **スキーマとマイグレーション**
   - `connections`, `projects`, `commits`, `user_filters` テーブルを作成。
   - インデックス（project_id, branch_name, committed_date_utc, author_email）を追加。
   - `schema.rs` の CURRENT_SCHEMA_VERSION と同期させる。

3. **リポジトリ実装の差し替え**
   - `ConnectionRepository`, `ProjectRepository`, `CommitRepository`, `UserFilterRepository` を SQLite 実装に切り替える。
   - `bulk_upsert` はトランザクション + prepared statement で一括挿入し、ユニーク制約で重複スキップ。

4. **JSON からの移行（既存ユーザー向け）**
   - 起動時に旧 JSON データがあれば SQLite にインポートし、成功後は JSON 書き込みを停止する。
   - 失敗時は既存 JSON を保持し、エラーメッセージを UI に表示する。

5. **Tauri コマンドの検証**
   - 既存コマンド（gitlab*connection_set、projects_sync、commits_collect、stats_monthly*\*、user_filter_get/user_filter_set）が SQLite を利用することを確認。
   - accessToken は UI/ログ非表示を維持し、SQLite から取得する。

## Test Plan

- Rust（storage/unit）
  - SQLite リポジトリの CRUD テスト
  - マイグレーション適用テスト
  - 10万件バッチ挿入の処理結果テスト（重複スキップ含む）
- Rust（command/contract）
  - 既存コマンドの契約テスト（入力/出力の互換性）
- Frontend（Vitest）
  - 既存 UI が接続/同期/収集/統計表示で動作することの回帰確認

## Local Verification

1. `cargo test`（Rust）
2. `npm test`（Frontend）
3. 手動フロー: 接続設定 → プロジェクト同期 → コミット収集 → 統計表示

## Notes

- accessToken の保存方式変更は UI/ログ非表示を前提に SQLite 保存へ切り替える。
- 大量データ操作時のキャンセル要求はトランザクションでロールバックする。
