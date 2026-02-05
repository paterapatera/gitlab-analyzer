# Tasks: 集計表示におけるユーザーフィルタリング

**Input**: Design documents from `/specs/001-stats-user-filter/`  
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/, quickstart.md

**Tests**: テストタスクは含まれていません（仕様で明示的に要求されていないため）

**Organization**: タスクはUser Storyごとにグループ化され、各Storyを独立して実装・テスト可能にします。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能（異なるファイル、依存関係なし）
- **[Story]**: このタスクが属するUser Story（US1, US2, US3）
- 説明には正確なファイルパスを含む

## Path Conventions

Tauri デスクトップアプリケーション構造（plan.md準拠）:

- Frontend: `src/` (React/TypeScript)
- Backend: `src-tauri/src/` (Rust)
- Tests: `src/test/` (frontend), `src-tauri/src/` (backend unit tests)

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: プロジェクトの初期化と基本構造の準備

- [x] T001 shadcn/ui Checkboxコンポーネントを追加（未存在の場合: `npx shadcn@latest add checkbox`）
- [x] T002 [P] shadcn/ui ScrollAreaコンポーネントを追加（未存在の場合: `npx shadcn@latest add scroll-area`）
- [x] T003 [P] UserFilterViewType型定義をTauri契約に追加 src/lib/contracts/tauriCommands.ts

**Checkpoint**: shadcn/uiコンポーネントと型定義が準備完了

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: 全User Story実装前に完了必須のコアインフラストラクチャ

**⚠️ CRITICAL**: このフェーズ完了まで、いかなるUser Story作業も開始できません

- [x] T004 UserFilterStorageデータ構造を定義 src-tauri/src/storage/user_filter_repository.rs（新規作成）
- [x] T005 UserFilterRepositoryの基本実装（new, get, setメソッド）src-tauri/src/storage/user_filter_repository.rs
- [x] T006 [P] user_filter_get Tauriコマンドを実装 src-tauri/src/commands/user_filter_get.rs（新規作成）
- [x] T007 [P] user_filter_set Tauriコマンドを実装 src-tauri/src/commands/user_filter_set.rs（新規作成）
- [x] T008 Tauriコマンドをmod.rsに登録 src-tauri/src/commands/mod.rs
- [x] T009 Tauriコマンドをmain.rsのinvoke_handlerに登録 src-tauri/src/main.rs
- [x] T010 user_filter_repositoryをstorage/mod.rsに登録 src-tauri/src/storage/mod.rs
- [x] T011 [P] getUserFilterState/setUserFilterState関数をTauri契約に追加 src/lib/contracts/tauriCommands.ts
- [x] T012 cargo testでBackend動作確認（user_filter_repository.rsにテストコード追加）

**Checkpoint**: Foundation ready - User Story実装が並列開始可能

---

## Phase 3: User Story 1 - ユーザーフィルタリングによる集計表示 (Priority: P1) 🎯 MVP

**Goal**: チェックボックスでユーザーを選択/解除し、グラフとテーブルの表示を絞り込む

**Independent Test**: 複数ユーザーのコミットデータが保存済みの状態で、チェックボックスを操作→グラフとテーブルの表示内容が選択されたユーザーのみに絞られることを手動確認

### Implementation for User Story 1

- [x] T013 [P] [US1] useUserFilter hookの基本実装（状態管理、永続化、復元ロジック）src/features/stats/useUserFilter.ts（新規作成）
- [x] T014 [P] [US1] UserFilterコンポーネントのUI実装（Checkbox一覧、アルファベット順ソート、ScrollArea統合）src/features/stats/UserFilter.tsx（新規作成）
- [x] T015 [US1] App.tsxにUserFilter統合（useUserFilter呼び出し、選択変更通知）src/App.tsx
- [x] T016 [US1] MonthlyBarChartにフィルタリングロジック適用（selectedUsersでデータ絞り込み）src/App.tsx
- [x] T017 [US1] MonthlyTableにフィルタリングロジック適用（selectedUsersでデータ絞り込み）src/App.tsx
- [x] T018 [US1] プロジェクト別ビューでcontextKey生成（`<project>/<branch>/<year>`形式）を実装・統合
- [x] T019 [US1] 横断ビューでcontextKey生成（`<year>`形式）を実装・統合
- [x] T020 [US1] 初期表示時のデフォルト全選択動作を確認・調整（useUserFilterのロジック検証）
- [x] T021 [US1] 選択ユーザー0件時のUI表示調整（「ユーザーを選択してください」メッセージ表示）

**Checkpoint**: User Story 1が完全に機能し、独立してテスト可能

---

## Phase 4: User Story 2 - 全選択/全解除の一括操作 (Priority: P2)

**Goal**: 全ユーザーを一括で選択または解除し、効率的にフィルタリング条件を変更

**Independent Test**: 10名以上のユーザーデータが保存済みの状態で、「全選択」「全解除」ボタンをクリック→全チェックボックスが一括操作されることを手動確認

### Implementation for User Story 2

- [x] T022 [P] [US2] useUserFilter hookにselectAll/deselectAllメソッドを実装 src/features/stats/useUserFilter.ts
- [x] T023 [US2] UserFilterコンポーネントに「全選択」「全解除」ボタンを追加 src/features/stats/UserFilter.tsx
- [x] T024 [US2] 一部選択状態からの全選択動作を検証・調整
- [x] T025 [US2] 全解除後のグラフ/テーブル空表示を確認

**Checkpoint**: User Story 1とUser Story 2が両方とも独立して動作

---

## Phase 5: User Story 3 - 選択状態の永続化 (Priority: P3)

**Goal**: 選択状態をアプリケーション内に保存し、同一条件で復元

**Independent Test**: 特定のユーザーを選択→他の条件に切り替え→元の条件に戻る→同じ選択状態が復元されることを手動確認

### Implementation for User Story 3

- [x] T026 [US3] useUserFilter hookの永続化ロジックを強化（setUserFilterState呼び出しのエラーハンドリング改善）src/features/stats/useUserFilter.ts
- [x] T027 [US3] プロジェクト/ブランチ/年切り替え時の選択状態復元動作を検証
- [x] T028 [US3] 横断ビュー切り替え時の独立選択状態を検証（FR-008要件）
- [x] T029 [US3] アプリ再起動後の選択状態復元を検証（user_filter_state.json読み込み確認）
- [x] T030 [US3] 存在しないユーザーが保存済みデータに含まれる場合の除外処理を確認

**Checkpoint**: 全User Storyが独立して機能

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: 複数User Storyに影響する改善

- [x] T031 [P] パフォーマンス検証（100名以上のユーザーでスクロール滑らかさ確認: SC-003）
- [x] T032 [P] リアルタイム更新速度検証（チェックボックス操作後1秒以内: SC-001）
- [x] T033 エッジケース検証（選択0件、大量ユーザー、特殊文字、同一ユーザー名）
- [x] T034 React.memoでUserFilterコンポーネントをメモ化（パフォーマンス最適化）
- [x] T035 [P] JSDoc/Rustドキュメントコメント追加（新規関数/型すべて）
- [x] T036 quickstart.md検証手順を実行（Verification Checklist全項目確認）
- [x] T037 [P] Constitution Principle準拠確認（コード品質、テスト、UX一貫性、再利用性）

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: 依存なし - 即座開始可能
- **Foundational (Phase 2)**: Setup完了に依存 - 全User Storyをブロック
- **User Stories (Phase 3-5)**: 全てFoundational完了に依存
  - User Story 1 (P1): Foundational後に開始可能 - 他Storyへの依存なし
  - User Story 2 (P2): Foundational後に開始可能 - US1の機能を拡張（独立テスト可能）
  - User Story 3 (P3): Foundational後に開始可能 - US1/US2の永続化を強化（独立テスト可能）
- **Polish (Phase 6)**: 全User Story完了に依存

### User Story Dependencies

- **User Story 1 (P1)**: Foundational完了後に開始 - 他Storyへの依存なし（コア機能）
- **User Story 2 (P2)**: Foundational完了後に開始 - US1のuseUserFilter/UserFilterを拡張するが独立テスト可能
- **User Story 3 (P3)**: Foundational完了後に開始 - US1の永続化ロジックを強化するが独立テスト可能

### Within Each User Story

- **US1**: useUserFilter/UserFilterコンポーネント並列作成 → StatsFilters統合 → MonthlyBarChart/Table適用
- **US2**: selectAll/deselectAllメソッド追加 → ボタンUI追加 → 動作検証
- **US3**: 永続化ロジック強化 → 各種切り替えシナリオ検証

### Parallel Opportunities

- Setup phase: T001, T002, T003（全て並列可能）
- Foundational phase: T006, T007（Tauriコマンド並列実装）、T011（Frontend契約並列実装）
- User Story 1: T013, T014（hook/UI並列作成）
- 複数開発者がいる場合: Foundational完了後、US1/US2/US3を並列実装可能

---

## Parallel Example: User Story 1

```bash
# User Story 1のコアコンポーネントを並列起動:
Task: "useUserFilter hookの基本実装 src/features/stats/useUserFilter.ts"
Task: "UserFilterコンポーネントのUI実装 src/features/stats/UserFilter.tsx"

# 完了後、統合タスクを順次実行:
Task: "StatsFiltersコンポーネントにUserFilter統合 src/features/stats/StatsFilters.tsx"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1完了: Setup（shadcn/ui準備）
2. Phase 2完了: Foundational（Backend永続化、Frontend契約）- **CRITICAL ブロッカー**
3. Phase 3完了: User Story 1（チェックボックス選択、フィルタリング、リアルタイム更新）
4. **STOP and VALIDATE**: User Story 1を独立テスト（複数ユーザーで選択/解除→グラフ/テーブル更新確認）
5. デプロイ/デモ可能（コア価値提供）

### Incremental Delivery

1. Setup + Foundational完了 → Foundation ready
2. User Story 1追加 → 独立テスト → デプロイ/デモ（**MVP!** - チェックボックス選択）
3. User Story 2追加 → 独立テスト → デプロイ/デモ（一括操作による効率化）
4. User Story 3追加 → 独立テスト → デプロイ/デモ（永続化で作業効率向上）
5. 各Storyが前のStoryを壊さず価値を追加

### Parallel Team Strategy

複数開発者がいる場合:

1. チーム全員でSetup + Foundational完了（T001-T012）
2. Foundational完了後:
   - Developer A: User Story 1（T013-T021）
   - Developer B: User Story 2（T022-T025）
   - Developer C: User Story 3（T026-T030）
3. 各Storyが独立して完了し、並列統合

---

## Notes

- **[P]** タスク = 異なるファイル、依存関係なし（並列実行可能）
- **[Story]** ラベル = User Storyへのトレーサビリティ確保
- 各User Storyは独立して完了・テスト可能
- 各タスクまたは論理グループ後にコミット
- 任意のCheckpointでStoryを独立検証可能
- 避けるべき: 曖昧なタスク、同一ファイル競合、Story独立性を壊す依存関係
- テストタスクは含まれていない（仕様で明示的に要求されていないため。必要に応じてPhase 6でT012のような単体テストを追加可能）

---

## Task Summary

- **Total Tasks**: 37タスク
- **Setup**: 3タスク
- **Foundational**: 9タスク（全User Storyをブロック）
- **User Story 1 (P1 - MVP)**: 9タスク
- **User Story 2 (P2)**: 4タスク
- **User Story 3 (P3)**: 5タスク
- **Polish**: 7タスク
- **Parallel Opportunities**: Setup全3タスク、Foundational内3タスク、US1内2タスク、Polish内4タスク（合計12タスク並列可能）

### Suggested MVP Scope

**MVP = User Story 1のみ（T001-T021）**:

- チェックボックスでユーザー選択/解除
- グラフ/テーブルのリアルタイムフィルタリング
- プロジェクト別/横断ビューでの独立選択状態
- 初期表示時の全選択デフォルト
- 選択0件時のメッセージ表示

このMVPでコア価値（欲しいユーザー情報だけで分析）を提供可能。US2（一括操作）とUS3（永続化）は後続増分として追加。
