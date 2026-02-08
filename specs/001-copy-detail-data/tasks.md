# Tasks: 詳細データのコピー

**Input**: Design documents from `/specs/001-copy-detail-data/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Include unit tests for TSV変換ロジック（Constitution IIに準拠）。

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Review detail table layout and insertion points in src/features/stats/StatsTab.tsx and src/features/stats/MonthlyTable.tsx

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core shared logic required by all user stories

- [x] T002 Add TSV変換ユーティリティ in src/features/stats/buildMonthlyTableTsv.ts
- [x] T003 Add unit tests for TSV変換 in src/test/features/stats/buildMonthlyTableTsv.test.ts

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Copy detail data from the panel (Priority: P1) 🎯 MVP

**Goal**: 詳細データ表の表示内容をTSV（ヘッダー行付き）でコピーし、スプレッドシートへ貼り付けできるようにする

**Independent Test**: コピーアイコンをクリックしてTSVが貼り付け可能で、列順/行順が表示と一致することを確認

### Implementation for User Story 1

- [x] T004 [US1] Add copy icon button to detail data panel header in src/features/stats/StatsTab.tsx
- [x] T005 [US1] Implement copy handler using buildMonthlyTableTsv and navigator.clipboard.writeText in src/features/stats/StatsTab.tsx
- [x] T006 [US1] Reuse shared header builder from buildMonthlyTableTsv in src/features/stats/MonthlyTable.tsx to keep TSV/table header alignment

**Checkpoint**: User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Confirm copy completion (Priority: P2)

**Goal**: コピー完了を詳細データパネル上部で2秒表示し、操作完了が分かるようにする

**Independent Test**: コピー後にパネル上部にメッセージが2秒表示されることを確認

### Implementation for User Story 2

- [x] T007 [US2] Add copy success message state and 2-second auto-dismiss in src/features/stats/StatsTab.tsx
- [x] T008 [US2] Show header-only copy message when no rows in src/features/stats/StatsTab.tsx

**Checkpoint**: User Story 2 should be fully functional and testable independently

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [ ] T009 Run quickstart validation steps in specs/001-copy-detail-data/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational phase completion
- **Polish (Final Phase)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational - Uses the copy flow from US1

### Within Each User Story

- Shared utilities before UI integration
- Copy behavior before confirmation messaging
- Story complete before moving to next priority

### Parallel Opportunities

- T002 and T003 can be done by separate developers if test implementation waits for the utility API to settle
- After Phase 2 completes, US1 and US2 work can proceed in parallel if copy handler API is agreed

---

## Parallel Example: User Story 1

```bash
Task: "Add copy icon button to detail data panel header in src/features/stats/StatsTab.tsx"
Task: "Reuse shared header builder from buildMonthlyTableTsv in src/features/stats/MonthlyTable.tsx"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Demo
3. Add User Story 2 → Test independently → Demo
4. Finish Polish tasks

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1 UI integration
   - Developer B: User Story 2 messaging
3. Stories complete and integrate independently
