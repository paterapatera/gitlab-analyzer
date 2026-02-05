# Research: 集計表示におけるユーザーフィルタリング

**Feature**: [spec.md](spec.md) | **Plan**: [plan.md](plan.md)  
**Date**: 2026-02-05

## Research Tasks

### 1. React状態管理パターン（ユーザー選択状態）

**Decision**: ローカル状態（useState） + カスタムhook（useUserFilter）を採用

**Rationale**:

- ユーザー選択状態は単一画面（stats）内のローカル状態であり、グローバル状態管理（Redux/Zustand）は過剰
- カスタムhookで選択ロジック・永続化・復元を抽象化し、テスタビリティと再利用性を確保
- 既存の`StatsFilters.tsx`パターン（年選択など）と一貫性を保つ

**Alternatives considered**:

- ❌ Redux/Zustand: グローバル状態管理は複雑性が高く、単一画面のフィルタには不要
- ❌ Context API: プロバイダー追加の手間があり、ローカル状態で十分
- ✅ useState + カスタムhook: シンプルで既存パターンと一貫

### 2. shadcn/uiコンポーネント選定

**Decision**: `Checkbox` + `ScrollArea` コンポーネントを使用

**Rationale**:

- `Checkbox`: shadcn/uiの標準コンポーネントでアクセシビリティ対応済み（ARIA属性、キーボード操作）
- `ScrollArea`: 50名以上のユーザー表示時にスクロール可能なコンテナを提供（Success Criteria SC-003）
- 既存のshadcn/ui使用パターン（Button, Card, Tableなど）と一貫

**Alternatives considered**:

- ❌ ネイティブ`<input type="checkbox">`: アクセシビリティ・スタイル統一の手間が増加
- ❌ カスタムCheckbox実装: Constitution Principle IIIに反する（shadcn優先）
- ✅ shadcn Checkbox + ScrollArea: アクセシビリティ・スタイル・パフォーマンスを兼備

### 3. 永続化ストレージ設計

**Decision**: `user_filter_state.json` ファイルに保存（既存のJSON Storeパターン）

**Rationale**:

- 既存の永続化パターン（`gitlab_connection.json`, `commits.json`など）と一貫
- データ構造: `{ "project-view": { "<project>/<branch>/<year>": ["user1", "user2"] }, "cross-view": { "<year>": ["user1", "user3"] } }`
- ビュー種別ごとに選択状態を独立管理（FR-008）
- プロジェクト/ブランチ/年の組み合わせをキーとして保存（Clarifications Q1）

**Alternatives considered**:

- ❌ SQLite: 単純なkey-value構造にはオーバースペック、既存パターンとの一貫性が低い
- ❌ localStorage（ブラウザAPI）: Tauriデスクトップアプリでは使用不可
- ✅ JSON Store: 既存実装と一貫、シンプルで十分

### 4. フィルタリングロジックの実装方法

**Decision**: フロントエンドでフィルタリング（Tauri backend は状態の永続化のみ）

**Rationale**:

- 集計データ（月次コミット行数）は既にTauri backendから取得済み（`stats_monthly_*_view` コマンド）
- フィルタリングはクライアント側で選択ユーザーに基づいてデータを絞り込む
- リアルタイム更新（FR-003）を実現するため、フロントエンドでの即座フィルタリングが最適
- Backend変更を最小化し、既存のstatsコマンドを変更せず再利用

**Alternatives considered**:

- ❌ Backend側でフィルタリング: 既存のstatsコマンドを大幅変更、リアルタイム更新が複雑化
- ✅ Frontend側でフィルタリング: シンプル、リアルタイム、既存Backend変更なし

### 5. ユーザー表示順序の実装

**Decision**: JavaScriptの`localeCompare`でアルファベット順ソート

**Rationale**:

- Clarifications Q2の決定: ユーザー名のアルファベット順（昇順）
- `localeCompare`は国際化対応（日本語、英語など）
- パフォーマンス: 100名程度のソートは十分高速（<10ms）

**Alternatives considered**:

- ❌ 単純な`sort()`: 国際化対応が不十分
- ✅ `localeCompare`: 国際化対応、標準API

### 6. パフォーマンス最適化戦略

**Decision**: React.memoでUserFilterコンポーネントをメモ化、仮想化は初期実装では不要

**Rationale**:

- Success Criteria SC-003: 100名以上でも滑らかなスクロール
- 100名のチェックボックスレンダリングは現代のブラウザで十分高速（<50ms）
- shadcn ScrollAreaで高さ制限を設定し、DOMノード数を削減
- 仮想化（react-window等）は複雑性が高く、初期実装では過剰
- 将来的に1000名以上のユーザーが必要になった場合は、仮想化を検討

**Alternatives considered**:

- ❌ 仮想化（react-window）: 100名程度では複雑性が不要
- ✅ React.memo + ScrollArea: シンプルで十分

## Implementation Notes

### Frontend (React/TypeScript)

**主要コンポーネント**:

- `UserFilter.tsx`: チェックボックス一覧表示、全選択/全解除ボタン
- `useUserFilter.ts`: 選択状態管理、永続化呼び出し、フィルタリング適用

**統合ポイント**:

- `StatsFilters.tsx`: UserFilterコンポーネントを追加
- `MonthlyBarChart.tsx` / `MonthlyTable.tsx`: フィルタ済みデータを受け取り表示

### Backend (Rust)

**新規Tauriコマンド**:

- `user_filter_get(view_type, context_key)`: 選択状態取得
- `user_filter_set(view_type, context_key, selected_users)`: 選択状態保存

**ストレージ実装**:

- `UserFilterRepository`: JSON Store経由で`user_filter_state.json`を読み書き
- 既存の`ConnectionRepository`, `CommitRepository`パターンを踏襲

### Testing Strategy

**Frontend**:

- `UserFilter.test.tsx`: チェックボックス選択/解除、全選択/全解除、UI表示
- `useUserFilter.test.ts`: 状態管理、フィルタリングロジック、永続化呼び出し

**Backend**:

- `user_filter_test.rs`: Repository CRUD操作、JSONシリアライズ/デシリアライズ

## Dependencies

**新規追加不要**: 既存の依存関係で実装可能

- shadcn/ui Checkbox: `shadcn add checkbox`で追加（既存パターン）
- shadcn/ui ScrollArea: `shadcn add scroll-area`で追加（既存パターン）

## Risks & Mitigations

| Risk                                     | Impact | Mitigation                                              |
| ---------------------------------------- | ------ | ------------------------------------------------------- |
| 1000名以上のユーザーでパフォーマンス低下 | Medium | 初期実装でパフォーマンス測定、必要に応じて仮想化導入    |
| 選択状態の永続化キー衝突                 | Low    | プロジェクト名/ブランチ名/年をURLエンコードしてキー生成 |
| ビュー切り替え時の選択状態混同           | Low    | ビュー種別（project-view/cross-view）を明示的に分離     |

## Open Questions

なし（Clarificationsセッションで全て解決済み）
