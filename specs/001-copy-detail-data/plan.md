# Implementation Plan: 詳細データのコピー

**Branch**: `001-copy-detail-data` | **Date**: 2026-02-08 | **Spec**: [spec.md](spec.md)
**Input**: Feature specification from `/specs/001-copy-detail-data/spec.md`

## Summary

集計表示の詳細データ表にコピーアイコンを追加し、表示中の行と順序をそのままTSV（ヘッダー行付き）でクリップボードへコピーできるようにする。コピー完了メッセージは詳細データパネル上部に2秒表示し、0行の場合はヘッダーのみをコピーする。プロジェクト別ビュー/横断ビューの両方で同一体験を提供する。実装はフロントエンドのみで完結する。

## Technical Context

**Language/Version**: TypeScript 5.6.2 (frontend), Rust 1.x (backend)  
**Primary Dependencies**: React 18.3, Tauri 2, Vite 6, Tailwind CSS 4, shadcn/ui, Vitest 4  
**Storage**: N/A（既存の集計データ表示のみを使用）  
**Testing**: Vitest (TypeScript), cargo test (Rust)  
**Target Platform**: Desktop（Windows/macOS/Linux via Tauri）
**Project Type**: Tauri デスクトップアプリ（React frontend + Rust backend）  
**Performance Goals**: 500行程度の表でも1秒以内にコピー完了  
**Constraints**: 既存の詳細データ表と表示順を保持、TSV形式（ヘッダー行付き）、コピー後メッセージはパネル上部に2秒表示  
**Scale/Scope**: 月次×ユーザーの表（最大100ユーザー×12ヶ月程度）

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Initial Check (Before Phase 0)

#### Principle I - コード品質

✅ **Pass** - TSV生成ロジックを小関数に分離し、日本語JSDocと必要な`NOTE:`コメントを付与する方針。

#### Principle II - テスト基準

✅ **Pass** - TSV生成ユーティリティとコピー実行の振る舞いをVitestでテスト。UI境界の手動検証手順も用意。

#### Principle III - UX の一貫性

✅ **Pass** - 詳細データパネル内の既存Card構造を維持し、shadcnのButton/Alertパターンで状態表示を統一。

#### Principle IV - MCP 活用

✅ **Pass** - serena MCPで既存StatsTab/MonthlyTableの参照を追跡し、UI追加時はshadcn MCPを利用。

#### Principle V - 再利用性・拡張性

✅ **Pass** - TSV変換を独立関数に分離し、将来的なCSV対応や他テーブルにも流用可能な設計にする。

#### Principle VI - セキュリティとプライバシー

✅ **Pass** - 表示名のみコピーし、authorEmailはUI/クリップボードに含めない方針を維持。

**Initial Overall**: ✅ All gates pass. No violations requiring justification.

---

### Post-Design Check (After Phase 1)

#### Principle I - コード品質

✅ **Pass** - data-model.mdでコピー対象と表構造を整理し、JSDoc/`NOTE:`の追加方針を明記。

#### Principle II - テスト基準

✅ **Pass** - quickstart.mdにユニットテストと手動検証手順を定義。

#### Principle III - UX の一貫性

✅ **Pass** - 詳細データパネル上部のメッセージ表示とアイコンボタンを既存Card表現に統一。

#### Principle IV - MCP 活用

✅ **Pass** - serena/shadcnの利用計画を明記。

#### Principle V - 再利用性・拡張性

✅ **Pass** - TSV生成を関数化し、月次表以外の詳細表にも適用可能。

#### Principle VI - セキュリティとプライバシー

✅ **Pass** - 機密情報を含めない列のみコピー対象とし、ログ出力も行わない。

**Post-Design Overall**: ✅ All gates pass. Design adheres to constitution principles.

## Project Structure

### Documentation (this feature)

```text
specs/001-copy-detail-data/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── tauri-commands.openapi.yaml
└── tasks.md             # Phase 2 output (not created here)
```

### Source Code (repository root)

```text
src/
├── features/
│   └── stats/
│       ├── StatsTab.tsx                # [MODIFY] 詳細データパネルにコピーUI追加
│       ├── MonthlyTable.tsx            # [MODIFY] TSV生成に必要なヘッダー/行抽出
│       └── buildMonthlyTableTsv.ts     # [ADD] 表データをTSVへ変換
├── components/
│   └── ui/
│       ├── button.tsx                  # [EXISTING] コピーアイコンボタン
│       └── alert.tsx                   # [EXISTING] コピー完了メッセージ
└── test/
    └── features/
        └── stats/
            └── buildMonthlyTableTsv.test.ts # [ADD] TSV変換ユニットテスト
```

**Structure Decision**: 既存のTauriデスクトップ構成を維持し、詳細データ表に関わるロジックは`src/features/stats/`へ集約する。コピー変換は専用ユーティリティに切り出し、UIから独立して再利用できる形にする。

## Complexity Tracking

> No violations. This section is intentionally left empty.
