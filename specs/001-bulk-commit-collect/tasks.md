---
description: 'Task list for Bulk Commit Continuation'
---

# Tasks: 一括コミット収集の継続

**Input**: Design documents from /specs/001-bulk-commit-collect/
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Required (Constitution II). Include backend + frontend tests.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: [ID] [P?] [Story] Description

- [P]: Can run in parallel (different files, no dependencies)
- [Story]: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Minimal project setup needed before implementation

- [x] T001 Add uuid dependency for run_id generation in src-tauri/Cargo.toml

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

- [x] T002 Add bulk collection tables migration (V6) in src-tauri/src/storage/sqlite/migrations.rs
- [x] T003 Register migration version and wiring in src-tauri/src/storage/schema.rs
- [x] T004 Implement bulk collection repository (runs/results CRUD) in src-tauri/src/storage/bulk_collection_repository.rs
- [x] T005 [P] Expose bulk collection repository module in src-tauri/src/storage/mod.rs
- [x] T006 [P] Add bulk collection data structs in src-tauri/src/storage/model.rs

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - ワンボタンで全対象の続きを収集 (Priority: P1) 🎯 MVP

**Goal**: 収集履歴のある全対象に対して、ワンボタンで一括収集を開始できる。

**Independent Test**: 収集履歴がある対象が複数ある状態で一括収集を開始し、全対象が順次処理されることを確認する。

### Tests for User Story 1

- [x] T007 [P] [US1] Add repository unit tests for run start/register/record in src-tauri/src/storage/bulk_collection_repository_test.rs
- [x] T008 [P] [US1] Add repository tests for pending target retrieval/resume in src-tauri/src/storage/bulk_collection_repository_test.rs
- [x] T009 [P] [US1] Add command-level tests for collect_commits_bulk and resume flow in src-tauri/src/commands/commits_collect_bulk_test.rs
- [x] T010 [P] [US1] Add command-level tests for cancel_bulk_collection in src-tauri/src/commands/commits_collect_bulk_test.rs

### Implementation for User Story 1

- [x] T011 [US1] Expose collect_commits_inner for reuse in src-tauri/src/commands/commits_collect.rs
- [x] T012 [US1] Add repository method to fetch pending targets for resume in src-tauri/src/storage/bulk_collection_repository.rs
- [x] T013 [US1] Implement collect_commits_bulk with resume-pending behavior in src-tauri/src/commands/commits_collect_bulk.rs
- [x] T014 [US1] Implement cancel_bulk_collection command and cancellation flag handling in src-tauri/src/commands/commits_collect_bulk.rs
- [x] T015 [US1] Register bulk collect + cancel commands in src-tauri/src/commands/mod.rs and src-tauri/src/lib.rs
- [x] T016 [US1] Implement BulkCollectCard start/cancel UI in src/features/collect/BulkCollectCard.tsx
- [x] T017 [US1] Mount BulkCollectCard in the collect tab in src/features/collect/CollectTab.tsx

**Checkpoint**: User Story 1 is functional and testable independently

---

## Phase 4: User Story 2 - 進捗と結果の可視化 (Priority: P2)

**Goal**: 一括収集の進捗と結果をユーザーが確認できる。

**Independent Test**: 実行中に進捗が表示され、完了後に結果サマリが表示されることを確認する。

### Tests for User Story 2

- [x] T018 [P] [US2] Add UI tests for progress display in src/features/collect/BulkCollectCard.test.tsx
- [x] T019 [P] [US2] Add repository tests for status summary queries in src-tauri/src/storage/bulk_collection_repository_test.rs

### Implementation for User Story 2

- [x] T020 [US2] Emit progress events during bulk processing in src-tauri/src/commands/commits_collect_bulk.rs
- [x] T021 [US2] Implement status query command (getBulkCollectionStatus) in src-tauri/src/commands/commits_collect_bulk.rs
- [x] T022 [US2] Add repository methods for status summary in src-tauri/src/storage/bulk_collection_repository.rs
- [x] T023 [US2] Listen to progress events and render progress bar in src/features/collect/BulkCollectCard.tsx
- [x] T024 [US2] Render per-target result summary using status data in src/features/collect/BulkCollectCard.tsx

**Checkpoint**: User Stories 1 and 2 are functional and testable independently

---

## Phase 5: User Story 3 - 失敗対象の再試行 (Priority: P3)

**Goal**: 失敗した対象のみを再試行できる。

**Independent Test**: 失敗対象がある実行の後、再試行で失敗対象のみが処理されることを確認する。

### Tests for User Story 3

- [x] T025 [P] [US3] Add repository tests for failed target retrieval in src-tauri/src/storage/bulk_collection_repository_test.rs
- [x] T026 [P] [US3] Add UI tests for retry flow in src/features/collect/BulkCollectCard.test.tsx

### Implementation for User Story 3

- [x] T027 [US3] Implement retryFailedTargets command in src-tauri/src/commands/commits_collect_bulk.rs
- [x] T028 [US3] Add repository method to fetch failed targets in src-tauri/src/storage/bulk_collection_repository.rs
- [x] T029 [US3] Add "retry failed" action in src/features/collect/BulkCollectCard.tsx

**Checkpoint**: All user stories are functional and independently testable

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T030 [P] Update quickstart validation notes in specs/001-bulk-commit-collect/quickstart.md
- [x] T031 [P] Add security note for error redaction in specs/001-bulk-commit-collect/research.md
- [x] T032 Run quickstart.md validation checklist in specs/001-bulk-commit-collect/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: Depend on Foundational completion
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: Starts after Foundational; no dependency on other stories
- **US2 (P2)**: Starts after Foundational; integrates with US1 results
- **US3 (P3)**: Starts after Foundational; depends on US2 status data

### Within Each User Story

- Tests first
- Repository/data access before command/UI logic
- Command before UI wiring

### Parallel Opportunities

- T005, T006 can run in parallel after T004
- T007, T008, T009, T010 can run in parallel
- T018 and T019 can run in parallel
- T025 and T026 can run in parallel

---

## Parallel Example: User Story 1

```bash
Task: "Add repository unit tests for run start/register/record in src-tauri/src/storage/bulk_collection_repository_test.rs"
Task: "Add command-level tests for collect_commits_bulk and resume flow in src-tauri/src/commands/commits_collect_bulk_test.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate US1 independently (backend + UI)

### Incremental Delivery

1. Add US1 → Validate
2. Add US2 → Validate
3. Add US3 → Validate

### Suggested MVP Scope

- **MVP**: User Story 1 only (bulk collect start + sequential processing + persistence)
