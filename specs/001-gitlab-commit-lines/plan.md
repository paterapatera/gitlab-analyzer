# Implementation Plan: GitLab 月次コミット行数分析

**Branch**: `001-gitlab-commit-lines` | **Date**: 2026-02-03 | **Spec**: `specs/001-gitlab-commit-lines/spec.md`
**Input**: Feature specification from `/specs/001-gitlab-commit-lines/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

- GitLab のベースURL/アクセストークンを登録し、プロジェクト一覧を同期し、任意のプロジェクト/ブランチ/期間でコミットを収集して端末ローカルに保存する。
- 保存済みコミットから、年/月/ユーザー単位で（追加行 + 削除行）を集計し、集合縦棒グラフと表で閲覧する（欠損 stats は 0 扱い + 欠損件数を表示）。
- GitLab API 呼び出し・永続化・集計は Rust（Tauri command）で実装し、フロントエンドは表示/操作に専念する（トークン/メールの露出回避）。

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**:
TypeScript ~5.6.2 / React 18.3 / Vite 6 / Rust 1.93.0 / Tauri 2 / Bun 1.3.8

詳細:

- Frontend: TypeScript ~5.6.2 / React 18.3 / Vite 6
- Backend: Rust 1.93.0 / Tauri 2
- JS runtime: Bun 1.3.8（`tauri.conf.json` の beforeDev/beforeBuild が `bun run ...`）

**Primary Dependencies**:

- Frontend: `react`, `@tauri-apps/api`, `recharts`, `tailwindcss` / shadcn（components.json あり）
- Backend: `tauri`, `serde`, `serde_json`

**Storage**:

- 端末ローカル（アプリデータディレクトリ）に JSON ファイル保存（MVP）

**Testing**:

- Frontend: Vitest（`vitest`）
- Backend: `cargo test`

**Target Platform**: デスクトップ（Tauri）。当面は Windows を主要ターゲット。
**Project Type**: Desktop app（Tauri: Rust backend + React frontend）

**Performance Goals**:

- 集計表示（フィルタ変更→描画更新）: 2秒以内（SC-002）

**Constraints**:

- トークン/作者メールは UI とログに出さない（FR-016/17/18/19）
- 欠損 stats は 0 扱いで集計し、欠損件数を表示（FR-022）
- 月判定は UTC（FR-024）

**Scale/Scope**:

- 個人利用を想定。大量コミットでも収集が破綻しないようページング/進捗を設計する。

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

- 原則 I（コメント規約）: 新規追加/変更した「クラス名・関数名（コンストラクタ除く）・プロパティ名・定数名」に日本語で説明コメントを付与する（TypeScript の `export`、Rust の `pub` は優先対象だが限定しない）。意図が特殊な箇所は `NOTE:` を付ける。
- 原則 II（テスト）: 追加/変更ロジックには Vitest / cargo test を同一コミットで追加する。例外は plan.md に根拠と手動検証手順を記録。
- 原則 III（UX）: 状態（読み込み/空/エラー/成功）を統一し、shadcn の UI を優先、キーボード操作/ラベルを最低基準として満たす。
- 原則 IV（MCP）: Serena を優先するが、本環境では Serena の language server 初期化エラーが発生しているため、探索は通常検索ツールで代替する（実装時に復旧できれば Serena に戻す）。
- 原則 V（拡張性）: 取得/保存/集計/表示の責務を分離し、後で SQLite 等へ移行可能な境界（Repository 風の抽象）を用意する。

## Project Structure

### Documentation (this feature)

```text
specs/001-gitlab-commit-lines/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

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

**Structure Decision**: Desktop app（Tauri）。GitLab API 通信/永続化/集計は `src-tauri/`、UI は `src/` に集約する。

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation                  | Why Needed         | Simpler Alternative Rejected Because |
| -------------------------- | ------------------ | ------------------------------------ |
| [e.g., 4th project]        | [current need]     | [why 3 projects insufficient]        |
| [e.g., Repository pattern] | [specific problem] | [why direct DB access insufficient]  |
