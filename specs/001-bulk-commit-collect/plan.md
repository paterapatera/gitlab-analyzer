# Implementation Plan: 一括コミット収集の継続

**Branch**: `001-bulk-commit-collect` | **Date**: 2026-02-06 | **Spec**: [specs/001-bulk-commit-collect/spec.md](specs/001-bulk-commit-collect/spec.md)
**Input**: Feature specification from [specs/001-bulk-commit-collect/spec.md](specs/001-bulk-commit-collect/spec.md)

## Summary

- 収集履歴がある全ての (project_id, branch_name) ペアについて、ワンボタンで続きを一括収集できる機能を追加する。
- 各対象を順次実行し、最後の収集時刻以降のコミットを収集、成功時にチェックポイントを更新する。
- 進捗表示（総対象数/完了数/失敗数）と結果サマリ（対象ごとの成功/失敗と新規コミット件数）を提供する。
- 処理済み結果は逐次保存され、中断時にも未処理分から再開可能とする。

## Technical Context

**Language/Version**: TypeScript ~5.6.2 / React 18.3 / Vite 6 / Rust (edition 2021) / Tauri 2, bun  
**Primary Dependencies**: React, @tauri-apps/api, Tauri, serde, reqwest, tokio, chrono, rusqlite, shadcn/ui
**Storage**: SQLite（commits テーブルの committed_date_utc 最大値をチェックポイントとして使用）  
**Testing**: Vitest（frontend）、cargo test（backend）  
**Target Platform**: Desktop（Tauri: Windows/macOS/Linux）  
**Project Type**: Tauri デスクトップアプリ（React frontend + Rust backend）  
**Performance Goals**: 100件の対象の90%が10分以内に完了、一括収集開始は10秒以内、進捗表示は5秒以内  
**Constraints**: 既存コマンド API と同等のエラーハンドリング、順次実行によるリソース管理、中断時の部分結果保存  
**Scale/Scope**: 個人利用を想定、最大100件程度の対象を一括処理

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Initial Check (Before Phase 0)

- ✅ **Code Quality**: 一括処理のループ/エラーハンドリング/状態管理に日本語コメントと `NOTE:` を付与する。
- ✅ **Testing**: 一括収集コマンド（Tauri）の単体テスト + フロントエンドコンポーネントのテストを追加する。
- ✅ **UX Consistency**: 既存の CollectTab/CollectForm/CollectResult のパターンを踏襲し、shadcn コンポーネントを使用する。
- ✅ **MCP Usage**: serena で既存コマンド/リポジトリを探索、shadcn で UI 追加（ボタン/進捗表示/結果サマリ）を行う。
- ✅ **Reusability**: 単一対象の収集ロジックを再利用し、一括処理は反復呼び出しとエラー集約のみ追加する。
- ✅ **Security & Privacy**: 一括収集の結果サマリにもaccessToken/authorEmail を含めない（プロジェクト名/ブランチ名のみ表示）。

### Post-Design Check (After Phase 1)

- ✅ **Code Quality**:
  - 一括処理ループは `process_bulk_collection` 関数に分割し、各対象の処理は `collect_commits_inner` を再利用。
  - 日本語コメントで各ステップ（対象取得、チェックポイント取得、結果記録など）の意図を明記。
  - `NOTE:` で rate limit 対策（100ms 待機）の理由を説明。
- ✅ **Testing**:
  - リポジトリ層（`bulk_collection_repository_test.rs`）で単体テスト。
  - フロントエンド（`BulkCollectCard.test.tsx`）でコンポーネントテスト。
  - quickstart.md に E2E シナリオとパフォーマンステストを明記。
- ✅ **UX Consistency**:
  - `BulkCollectCard` は既存の `CollectTab` に統合され、shadcn コンポーネント（Card, Button, Progress）を使用。
  - 進捗表示と結果サマリは既存の状態表示パターンに準拠。
  - キーボード操作（Enter でボタン実行）とアクセシビリティを確保。
- ✅ **MCP Usage**:
  - serena で既存の `commits_collect.rs` と `commit_repository.rs` を探索し、`collect_commits_inner` の再利用を決定。
  - shadcn で `Progress` コンポーネントを追加（進捗バー）。
- ✅ **Reusability**:
  - `collect_commits_inner` を公開して再利用（Open/Closed 原則）。
  - リポジトリ層を独立した関数に分割（start_run, register_targets, record_result, complete_run など）。
  - 失敗対象の再試行は既存の一括収集ロジックを再利用可能。
- ✅ **Security & Privacy**:
  - 結果サマリには project_id と branch_name のみ表示（accessToken/authorEmail は含めない）。
  - SQLite に保存される bulk_collection_results テーブルにも機密情報は含まれない。
  - エラーメッセージから accessToken を除外（quickstart.md に明記）。

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
src/                       # React UI
├── App.tsx
├── main.tsx
└── features/
    └── collect/
        ├── CollectForm.tsx      # 単一対象の収集フォーム（既存）
        ├── CollectResult.tsx    # 単一対象の結果表示（既存）
        ├── CollectTab.tsx       # 収集タブ（既存）
        └── (new) BulkCollectCard.tsx  # 一括収集UI（ボタン、進捗、結果サマリ）

src-tauri/                 # Rust (Tauri backend)
├── src/
│   ├── commands/
│   │   ├── commits_collect.rs   # 単一対象の収集（既存）
│   │   └── (new) commits_collect_bulk.rs  # 一括収集コマンド
│   ├── storage/
│   │   ├── commit_repository.rs  # コミット CRUD（既存）
│   │   └── (new) bulk_collection_repository.rs  # 一括収集の状態/結果管理
│   └── domain/
└── tauri.conf.json
```

**Structure Decision**: Tauri デスクトップアプリ構成を維持し、一括収集は既存の `collect` フィーチャー配下に追加する。バックエンドは新しいコマンド（`commits_collect_bulk`）とリポジトリ（`bulk_collection_repository`）を追加する。

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation                  | Why Needed         | Simpler Alternative Rejected Because |
| -------------------------- | ------------------ | ------------------------------------ |
| [e.g., 4th project]        | [current need]     | [why 3 projects insufficient]        |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient]  |
