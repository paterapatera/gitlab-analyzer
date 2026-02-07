# Implementation Plan: プロジェクト選択のオートコンプリート

**Branch**: `001-project-autocomplete` | **Date**: 2026-02-07 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-project-autocomplete/spec.md`

## Summary

コミット収集画面と集計表示（プロジェクト別）で、プロジェクト選択をオートコンプリート化する。取得済みプロジェクト一覧をクライアント側で部分一致検索し、150msデバウンス/1文字以上で絞り込みを開始する。候補は入力欄下のドロップダウン（コンボボックス）で最大100件表示し、キーボード操作（上下矢印/Enter/Escape）に対応する。UIはshadcnの`Command` + `Popover`パターンを採用する。

## Technical Context

**Language/Version**: TypeScript 5.6.2 (frontend), Rust 1.x (backend)  
**Primary Dependencies**: React 18.3, Tauri 2, Vite 6, Tailwind CSS 4, shadcn/ui, Vitest 4  
**Storage**: 既存のローカル保存（プロジェクト一覧は既存コマンドで取得）  
**Testing**: Vitest (TypeScript), cargo test (Rust)  
**Target Platform**: Desktop（Windows/macOS/Linux via Tauri）
**Project Type**: Tauri デスクトップアプリ（React frontend + Rust backend）  
**Performance Goals**: 3文字入力時1秒以内の候補更新、100件以上でも滑らかに反応  
**Constraints**: クライアント側絞り込み、150msデバウンス、最大100件表示、case-insensitive、a11y対応  
**Scale/Scope**: 数十〜数百件のプロジェクト、2画面（コミット収集・集計）に適用

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Initial Check (Before Phase 0)

#### Principle I - コード品質

✅ **Pass** - 追加/変更した関数とコンポーネントに日本語JSDocを付与。複雑なフィルタリング条件は`NOTE:`で意図を明記。

#### Principle II - テスト基準

✅ **Pass** - フィルタロジックとUI操作にVitestを追加。Tauri境界は既存契約に依存し、必要に応じてモック。

#### Principle III - UX の一貫性

✅ **Pass** - shadcnの`Command`/`Popover`を採用し、ロード/空/エラーの表現を既存パターンに合わせる。キーボード操作とフォーカス可視化を担保。

#### Principle IV - MCP 活用

✅ **Pass** - serena MCPで既存UIを探索・参照追跡し、shadcn MCPでUIコンポーネントを追加する。

#### Principle V - 再利用性・拡張性

✅ **Pass** - フィルタ関数を独立し、ProjectAutocompleteコンポーネントを両画面で再利用できる設計にする。

#### Principle VI - セキュリティとプライバシー

✅ **Pass** - 機密情報をUI/ログに出力しない。プロジェクト一覧のみを扱い、入力値はクライアント側で処理。

**Initial Overall**: ✅ All gates pass. No violations requiring justification.

---

### Post-Design Check (After Phase 1)

#### Principle I - コード品質

✅ **Pass** - data-model.mdで状態管理を整理し、実装時のJSDoc/`NOTE:`追加方針を明記。

#### Principle II - テスト基準

✅ **Pass** - quickstart.mdでUI/フィルタリングのテスト・手動検証手順を明記。

#### Principle III - UX の一貫性

✅ **Pass** - shadcnのcomboboxパターンに統一し、空/上限/ローディング文言を設計。

#### Principle IV - MCP 活用

✅ **Pass** - shadcn MCPで`Command`/`Popover`追加方針、serena MCPで既存参照を追跡。

#### Principle V - 再利用性・拡張性

✅ **Pass** - ProjectAutocomplete + フィルタ関数の分離で再利用可能。

#### Principle VI - セキュリティとプライバシー

✅ **Pass** - UIで機密情報を扱わず、既存コマンドの結果のみ表示。

**Post-Design Overall**: ✅ All gates pass. Design adheres to constitution principles.

## Project Structure

### Documentation (this feature)

```text
specs/001-project-autocomplete/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/
│   └── tauri-commands.openapi.yaml
└── tasks.md             # Phase 2 output (not created here)
```

### Source Code (repository root)

```text
src/
├── components/
│   └── ui/
│       ├── command.tsx           # [ADD] shadcn Command
│       └── popover.tsx           # [ADD] shadcn Popover
├── features/
│   ├── projects/
│   │   ├── ProjectAutocomplete.tsx  # [ADD] オートコンプリート本体
│   │   ├── projectFilter.ts         # [ADD] フィルタ関数
│   │   └── ProjectsPanel.tsx        # [MODIFY] 検索入力 + ドロップダウン
│   └── stats/
│       └── ProjectBranchSelector.tsx # [MODIFY] プロジェクト選択を置換
└── test/
    └── features/
        └── projects/
            ├── projectFilter.test.ts        # [ADD]
            └── ProjectAutocomplete.test.tsx # [ADD]

src-tauri/
└── src/
    └── commands/                   # [NO CHANGE]
```

**Structure Decision**: 既存のTauriデスクトップ構造を維持し、`src/features/projects/`に再利用可能なオートコンプリートを集約。UIの共通部品は`src/components/ui/`に追加する。

## Complexity Tracking

> No violations. This section is intentionally left empty.
