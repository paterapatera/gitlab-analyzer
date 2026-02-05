---
description: 'Task list for SQLiteストレージへの移行'
---

# Tasks: SQLiteストレージへの移行

**Input**: Design documents from `specs/001-json-to-sqlite/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: 既存の憲章に従い、変更ロジックには Rust/Vitest のテストを追加する。

**Organization**: User Story ごとに独立実装/検証できるように分割。

## Format: `[ID] [P?] [Story] Description`

- **[P]**: 並列実行可能（別ファイル・非依存）
- **[Story]**: US1/US2 のみ（Setup/Foundational/Polish は付与しない）
- 各タスクは必ずファイルパスを含める

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: 依存追加と作業基盤の整備

- [x] T001 Update Rust dependencies for SQLite + secure storage in src-tauri/Cargo.toml
- [x] T002 [P] Add SQLite module folder structure in src-tauri/src/storage/sqlite/
- [x] T003 [P] Add test fixtures folder in src-tauri/src/storage/sqlite/tests/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: SQLite 接続・マイグレーション基盤の整備（以後の全作業をブロック）

- [x] T004 Implement DB path resolution and connection factory in src-tauri/src/storage/sqlite/db.rs
- [x] T005 Implement PRAGMA setup (WAL, busy_timeout, foreign_keys) in src-tauri/src/storage/sqlite/db.rs
- [x] T006 Implement migration runner and schema_migrations table in src-tauri/src/storage/sqlite/migrations.rs
- [x] T007 Wire sqlite module exports in src-tauri/src/storage/sqlite/mod.rs
- [x] T008 Add SQLite init hook on app startup in src-tauri/src/lib.rs

**Checkpoint**: SQLite の接続/マイグレーションが起動時に走る

---

## Phase 3: User Story 1 - SQLiteストレージレイヤーの実装 (Priority: P1) 🎯 MVP

**Goal**: JSON 永続化を SQLite に置換し、既存コマンドの動作互換を維持する

**Independent Test**: 新規インストールで GitLab 接続→同期→収集→統計表示が動作し、JSON が作成されない

### Tests for User Story 1

- [x] T009 [P] [US1] Add migration tests in src-tauri/src/storage/sqlite/tests/migrations_test.rs
- [x] T010 [P] [US1] Add ConnectionRepository SQLite tests in src-tauri/src/storage/sqlite/tests/connection_repository_test.rs
- [x] T011 [P] [US1] Add ProjectRepository SQLite tests in src-tauri/src/storage/sqlite/tests/project_repository_test.rs
- [x] T012 [P] [US1] Add CommitRepository SQLite tests in src-tauri/src/storage/sqlite/tests/commit_repository_test.rs
- [x] T013 [P] [US1] Add UserFilterRepository SQLite tests in src-tauri/src/storage/sqlite/tests/user_filter_repository_test.rs

### Implementation for User Story 1

- [x] T014 [US1] Create initial schema migration SQL in src-tauri/src/storage/sqlite/migrations/001_init.sql
- [x] T015 [US1] Implement SQLite schema application in src-tauri/src/storage/sqlite/migrations.rs
- [x] T016 [US1] Implement SQLite ConnectionRepository in src-tauri/src/storage/sqlite/connection_repository.rs
- [x] T017 [US1] Implement SQLite ProjectRepository in src-tauri/src/storage/sqlite/project_repository.rs
- [x] T018 [US1] Implement SQLite CommitRepository with unique constraint handling in src-tauri/src/storage/sqlite/commit_repository.rs
- [x] T019 [US1] Implement SQLite UserFilterRepository in src-tauri/src/storage/sqlite/user_filter_repository.rs
- [x] T021 [US1] Switch storage module to SQLite repos in src-tauri/src/storage/mod.rs
- [x] T022 [US1] Update gitlab_connection_get/set to use SQLite repos in src-tauri/src/commands/gitlab_connection_get.rs
- [x] T023 [US1] Update gitlab_connection_set to use SQLite repos in src-tauri/src/commands/gitlab_connection_set.rs
- [x] T024 [US1] Update projects_list/projects_sync to use SQLite repos in src-tauri/src/commands/projects_list.rs
- [x] T025 [US1] Update projects_sync to use SQLite repos in src-tauri/src/commands/projects_sync.rs
- [x] T026 [US1] Update commits_collect to use SQLite repos and transaction in src-tauri/src/commands/commits_collect.rs
- [x] T027 [US1] Update stats*monthly*\* to query SQLite in src-tauri/src/commands/stats_monthly_project_view.rs
- [x] T028 [US1] Update stats_monthly_cross_view to query SQLite in src-tauri/src/commands/stats_monthly_cross_view.rs
- [x] T029 [US1] Update user_filter_get/set to use SQLite in src-tauri/src/commands/user_filter_get.rs
- [x] T030 [US1] Update user_filter_set to use SQLite in src-tauri/src/commands/user_filter_set.rs
- [x] T031 [US1] Ensure JSON store is no longer written in src-tauri/src/storage/json_store.rs

**Checkpoint**: JSON 永続化が停止し、既存 UI が SQLite 経由で動作する

---

## Phase 4: User Story 2 - ストレージパフォーマンスの最適化とモニタリング (Priority: P2)

**Goal**: 100万件規模でも検索/集計が 5 秒以内で完了し、容量監視とキャンセルが可能

**Independent Test**: 100万件のテストデータで主要クエリが 5 秒以内、容量超過警告が UI に表示される

### Tests for User Story 2

- [ ] T032 [P] [US2] Add performance query tests in src-tauri/src/storage/sqlite/tests/performance_query_test.rs
- [ ] T033 [P] [US2] Add db size warning tests in src-tauri/src/storage/sqlite/tests/db_size_warning_test.rs

### Implementation for User Story 2

- [ ] T034 [US2] Add performance indexes in src-tauri/src/storage/sqlite/migrations/002_perf_indexes.sql
- [ ] T035 [US2] Implement batch insert with prepared statements in src-tauri/src/storage/sqlite/commit_repository.rs
- [ ] T036 [US2] Add query optimizations for stats aggregation in src-tauri/src/storage/sqlite/commit_repository.rs
- [ ] T037 [US2] Implement db size checker in src-tauri/src/storage/sqlite/health.rs
- [ ] T038 [US2] Add storage health command in src-tauri/src/commands/storage_health.rs
- [ ] T039 [US2] Wire storage health command in src-tauri/src/commands/mod.rs
- [ ] T040 [US2] Add storage warning UI in src/features/stats/StorageHealthNotice.tsx
- [ ] T041 [US2] Integrate storage warning into stats tab in src/features/stats/StatsTab.tsx
- [ ] T042 [US2] Add cancellation support for commit collection in src-tauri/src/commands/commits_collect.rs
- [ ] T043 [US2] Add cancel UI action in src/features/collect/CollectTab.tsx

**Checkpoint**: 大量データでも性能維持 + 容量警告 + 収集キャンセルが可能

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: ドキュメント/契約/回帰整理

- [ ] T044 [P] Update SQLite contract notes in specs/001-json-to-sqlite/contracts/tauri-commands.openapi.yaml
- [ ] T045 [P] Update quickstart verification steps in specs/001-json-to-sqlite/quickstart.md
- [ ] T046 Remove JSON-only references in specs/001-json-to-sqlite/research.md
- [ ] T047 Run quickstart.md validation steps and record notes in specs/001-json-to-sqlite/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies
- **Foundational (Phase 2)**: Depends on Setup completion
- **User Story 1 (Phase 3)**: Depends on Foundational
- **User Story 2 (Phase 4)**: Depends on Foundational (can start after US1 if staffing is limited)
- **Polish (Phase 5)**: Depends on US1/US2 completion

### User Story Dependencies

- **US1 (P1)**: 必須。SQLite 基盤の完成で MVP 到達
- **US2 (P2)**: US1 が完了している前提で性能最適化と監視を追加

### Parallel Opportunities

- Setup: T002/T003
- US1 tests: T009〜T013
- US2 tests: T032〜T033

---

## Parallel Example: User Story 1

```bash
Task: "T009 Add migration tests in src-tauri/src/storage/sqlite/tests/migrations_test.rs"
Task: "T010 Add ConnectionRepository SQLite tests in src-tauri/src/storage/sqlite/tests/connection_repository_test.rs"
Task: "T011 Add ProjectRepository SQLite tests in src-tauri/src/storage/sqlite/tests/project_repository_test.rs"
Task: "T012 Add CommitRepository SQLite tests in src-tauri/src/storage/sqlite/tests/commit_repository_test.rs"
Task: "T013 Add UserFilterRepository SQLite tests in src-tauri/src/storage/sqlite/tests/user_filter_repository_test.rs"
```

---

## Parallel Example: User Story 2

```bash
Task: "T032 Add performance query tests in src-tauri/src/storage/sqlite/tests/performance_query_test.rs"
Task: "T033 Add db size warning tests in src-tauri/src/storage/sqlite/tests/db_size_warning_test.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Phase 1 → Phase 2 完了
2. Phase 3 (US1) を実装し独立テスト
3. JSON が生成されないことを確認して MVP 完了

### Incremental Delivery

1. US1 完了後に US2 の性能/監視/キャンセルを追加
2. 各ストーリーごとに独立テストを実施
