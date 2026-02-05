# Implementation Plan: 集計表示におけるユーザーフィルタリング

**Branch**: `001-stats-user-filter` | **Date**: 2026-02-05 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-stats-user-filter/spec.md`

## Summary

月次コミット行数の集計表示画面（プロジェクト別ビュー・横断ビュー）に、チェックボックスによるユーザーフィルタリング機能を追加する。ユーザーは表示するユーザーを選択/解除でき、グラフとテーブルがリアルタイムで更新される。選択状態はプロジェクト/ブランチ/年の組み合わせごとに永続化され、アプリケーション再起動後も復元される。

## Technical Context

**Language/Version**: TypeScript 5.6.2 (frontend), Rust 1.x (backend)  
**Primary Dependencies**: React 18.3, Tauri 2, Recharts 3.7, shadcn/ui, Vitest 4.0  
**Storage**: JSON ファイルストレージ（既存: `src-tauri/src/storage/json_store.rs`）  
**Testing**: Vitest (TypeScript), cargo test (Rust)  
**Target Platform**: Desktop（Windows/macOS/Linux via Tauri）  
**Project Type**: Tauri デスクトップアプリケーション（React frontend + Rust backend）  
**Performance Goals**: チェックボックス操作後1秒以内にグラフ/テーブル更新、100名以上のユーザーでも滑らかなスクロール  
**Constraints**: 既存のUI/UXパターンとの一貫性維持、shadcn/uiコンポーネント優先  
**Scale/Scope**: ユーザー数 100名以上を想定、2つのビュー（プロジェクト別・横断）に適用

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Initial Check (Before Phase 0)

#### Principle I - コード品質

✅ **Pass** - 新規コンポーネント/関数には日本語コメント（JSDoc/`///`）を追加予定。複雑なロジック（選択状態の永続化キー生成など）には`NOTE:`コメントで意図を明記。

#### Principle II - テスト基準

✅ **Pass** - 新規/変更したロジック（チェックボックス一覧表示、選択状態管理、フィルタリング適用）にはVitest/cargo testを追加予定。I/O境界（Tauriコマンド、永続化）には適切なモック/契約テストを用意。

#### Principle III - UX の一貫性

✅ **Pass** - shadcn/uiのCheckboxコンポーネントを使用予定（未存在の場合は追加）。既存のStatsFilters UIパターンに統一し、読み込み/空/エラー状態の表現を既存実装と揃える。

#### Principle IV - MCP 活用

✅ **Pass** - serena MCPで既存コード探索・参照追跡・局所編集を実施予定。shadcn MCPでCheckboxコンポーネント追加（必要に応じて）。

#### Principle V - 再利用性・拡張性

✅ **Pass** - ユーザーフィルタリングロジックを独立した関数/hookに分離予定。選択状態管理は汎用的に設計し、将来の他フィルタ追加にも対応可能にする。

**Initial Overall**: ✅ All gates pass. No violations requiring justification.

---

### Post-Design Check (After Phase 1)

#### Principle I - コード品質

✅ **Pass** - research.md, data-model.md, quickstart.mdで詳細な設計を文書化。実装ガイドでJSDoc/`///`コメント規約を明示。

#### Principle II - テスト基準

✅ **Pass** - quickstart.mdでテスト実装手順を明記（UserFilter.test.tsx, useUserFilter.test.ts, user_filter_test.rs）。I/O境界（Tauri契約）のテスト戦略を定義。

#### Principle III - UX の一貫性

✅ **Pass** - shadcn/ui Checkbox + ScrollAreaを採用。既存のStatsFiltersパターンに統合。読み込み/空状態の表現を設計（quickstart.mdで明示）。

#### Principle IV - MCP 活用

✅ **Pass** - 設計段階でserena MCPによる既存コード探索を実施。shadcn MCP活用方針を明記（Checkbox/ScrollArea追加）。

#### Principle V - 再利用性・拡張性

✅ **Pass** - useUserFilter hookで状態管理を抽象化。UserFilterコンポーネントは独立して再利用可能。将来の他フィルタ追加に対応可能な設計。

**Post-Design Overall**: ✅ All gates pass. Design adheres to constitution principles.

## Project Structure

### Documentation (this feature)

```text
specs/001-stats-user-filter/
├── plan.md              # This file
├── research.md          # Phase 0 output (技術調査)
├── data-model.md        # Phase 1 output (データモデル設計)
├── quickstart.md        # Phase 1 output (実装ガイド)
├── contracts/           # Phase 1 output (API契約)
│   └── user-filter-state.schema.json
└── tasks.md             # Phase 2 output (未作成)
```

### Source Code (repository root)

```text
src/
├── components/
│   └── ui/
│       ├── checkbox.tsx           # [ADD] shadcn Checkbox (if not exists)
│       └── scroll-area.tsx        # [ADD] shadcn ScrollArea (if not exists)
├── features/
│   └── stats/
│       ├── StatsFilters.tsx       # [MODIFY] ユーザーフィルタ追加
│       ├── UserFilter.tsx         # [ADD] ユーザー選択UI
│       ├── MonthlyBarChart.tsx    # [MODIFY] フィルタ適用
│       ├── MonthlyTable.tsx       # [MODIFY] フィルタ適用
│       └── useUserFilter.ts       # [ADD] フィルタ状態管理hook
├── lib/
│   └── contracts/
│       └── tauriCommands.ts       # [MODIFY] ユーザーフィルタ状態のTauri契約追加
└── test/
    ├── features/
    │   └── stats/
    │       └── UserFilter.test.tsx    # [ADD] UIコンポーネントテスト
    └── lib/
        └── useUserFilter.test.ts      # [ADD] hookテスト

src-tauri/
├── src/
│   ├── commands/
│   │   ├── user_filter_get.rs     # [ADD] フィルタ状態取得
│   │   └── user_filter_set.rs     # [ADD] フィルタ状態保存
│   └── storage/
│       └── user_filter_repository.rs # [ADD] フィルタ状態永続化
└── tests/
    └── user_filter_test.rs        # [ADD] フィルタ状態テスト

```

**Structure Decision**: Tauri デスクトップアプリケーション構造を採用。フロントエンド（React/TypeScript）とバックエンド（Rust）の明確な分離。既存の`src/features/stats/`に新UIコンポーネントを追加し、永続化ロジックは`src-tauri/src/storage/`に配置。

## Complexity Tracking

> No violations. This section is intentionally left empty.
