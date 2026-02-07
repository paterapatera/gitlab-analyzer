# Implementation Plan: Delete Collected Branch Commits

**Branch**: `001-delete-branch-commits` | **Date**: 2026-02-07 | **Spec**: `specs/001-delete-branch-commits/spec.md`
**Input**: Feature specification from `/specs/001-delete-branch-commits/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

- プロジェクト/ブランチ単位で収集済みコミットを削除し、集計ビューから除外する。
- 削除前に影響件数と集計影響を提示し、取り消し不可の確認ダイアログで確定する。
- 収集中の同一ブランチは削除をブロックし、理由を明示する。削除ボタンはゴミ箱アイコン。

## Technical Context

**Language/Version**: TypeScript ~5.6.2 / React 18.3 / Vite 6 / Rust 1.93.0 / Tauri 2 / Bun 1.3.x

**Primary Dependencies**:

- Frontend: `react`, `@tauri-apps/api`, `lucide-react`, `tailwindcss` / shadcn
- Backend: `tauri`, `serde`, `serde_json`, `rusqlite`

**JS Runtime**:

- Bun（`tauri.conf.json` の `beforeDevCommand/beforeBuildCommand` で使用）

**Storage**:

- 端末ローカル SQLite（`rusqlite`）

**Testing**:

- Frontend: Vitest（`vitest`）
- Backend: `cargo test`

**Target Platform**: デスクトップ（Tauri）。当面は Windows を主要ターゲット。
**Project Type**: Desktop app（Tauri: Rust backend + React frontend）

**Performance Goals**:

- 削除確定から集計反映まで 10 秒以内（SC-002）

**Constraints**:

- 物理削除で復元不可（FR-008）
- 収集中の同一ブランチは削除不可（FR-007）
- 既存の集計ビューから除外（FR-004）
- 画面/ログに機密情報を出さない（憲章 VI）

**Scale/Scope**:

- 個人利用を想定。単一端末・単一ユーザー。

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

- [x] **Code Quality**: 新規/変更シンボルに日本語 JSDoc/`///` を付与し、複雑な処理は分割+`NOTE:` で意図を明記する。
- [x] **Testing**: 影響集計・削除処理に対して Vitest / cargo test を追加し、例外は plan.md に根拠と手動検証を記録する。
- [x] **UX Consistency**: shadcn を優先し、状態表示（空/エラー/成功）と a11y を統一する。
- [x] **MCP Usage**: 探索は `serena`、UI は `shadcn` を優先し、例外は理由を残す。
- [x] **Reusability**: 影響計算・削除実行・UI 表示を分離し、後続の拡張に備える。
- [x] **Security & Privacy**: `accessToken`/`authorEmail` を UI/ログに表示せず、入力サニタイズを行う。

**Post-Design Check**: PASS（Phase 1 成果物に沿って再確認済み）

## Project Structure

### Documentation (this feature)

```text
specs/001-delete-branch-commits/
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
├── assets/
└── lib/
  └── utils.ts

src-tauri/                 # Rust (Tauri backend)
├── src/
│   ├── lib.rs
│   └── main.rs
└── tauri.conf.json
```

**Structure Decision**: Desktop app（Tauri）。削除/影響計算のロジックは `src-tauri/`、UI は `src/` に集約する。

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
| --------- | ---------- | ------------------------------------ |
| N/A       | N/A        | N/A                                  |
