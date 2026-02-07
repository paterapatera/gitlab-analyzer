# Tasks: Delete Collected Branch Commits

**Input**: Design documents from `/specs/001-delete-branch-commits/`
**Prerequisites**: plan.md (required), spec.md (required), research.md, data-model.md, contracts/

**Tests**: Not requested in the feature specification. Omit test tasks unless requested.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: UI primitives needed across the delete flow

- [X] T001 Add shadcn dialog component in src/components/ui/dialog.tsx

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared types and backend infrastructure required by all stories

- [X] T002 [P] Add delete command types and wrappers in src/lib/contracts/tauriCommands.ts
- [X] T003 [P] Add delete impact command module in src-tauri/src/commands/commits_branch_delete_impact.rs
- [X] T004 [P] Add delete command module in src-tauri/src/commands/commits_branch_delete.rs
- [X] T005 Update command registry in src-tauri/src/commands/mod.rs
- [X] T006 Update Tauri invoke handler in src-tauri/src/lib.rs
- [X] T007 [P] Add commit count/delete helpers in src-tauri/src/storage/commit_repository.rs
- [X] T008 [P] Add SQLite count/delete helpers in src-tauri/src/storage/sqlite/commit_repository.rs
- [X] T009 [P] Add running-collection guard helpers in src-tauri/src/storage/bulk_collection_repository.rs

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Remove miscollected branch data (Priority: P1) 🎯 MVP

**Goal**: Delete all collected commits for a selected branch and refresh aggregates

**Independent Test**: Delete a target branch and confirm aggregates exclude that branch afterward

### Implementation for User Story 1

- [X] T010 [US1] Implement delete command logic in src-tauri/src/commands/commits_branch_delete.rs
- [X] T011 [US1] Add delete action hook in src/features/stats/useBranchDelete.ts
- [X] T012 [US1] Add delete dialog UI with trash icon in src/features/stats/BranchDeleteDialog.tsx
- [X] T013 [US1] Wire delete action into stats filters in src/features/stats/StatsFilterCard.tsx
- [X] T014 [US1] Refresh stats after deletion in src/features/stats/StatsTab.tsx

**Checkpoint**: User Story 1 is fully functional and testable independently

---

## Phase 4: User Story 2 - Verify deletion impact before confirming (Priority: P2)

**Goal**: Show pre-confirmation impact summary and block deletion when required

**Independent Test**: View impact summary and complete deletion with the reported counts

### Implementation for User Story 2

- [X] T015 [US2] Implement impact summary logic in src-tauri/src/commands/commits_branch_delete_impact.rs
- [X] T016 [US2] Add impact summary hook in src/features/stats/useBranchDeleteImpact.ts
- [X] T017 [US2] Display impact summary and block reasons in src/features/stats/BranchDeleteDialog.tsx
- [X] T018 [US2] Show non-destructive "no commits" message in src/features/stats/BranchDeleteDialog.tsx

**Checkpoint**: User Stories 1 and 2 both work independently

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Documentation alignment and validation

- [X] T019 [P] Align command names with contracts in specs/001-delete-branch-commits/contracts/tauri-commands.openapi.yaml
- [X] T020 Run quickstart validation steps in specs/001-delete-branch-commits/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies
- **Foundational (Phase 2)**: Depends on Setup completion - blocks all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational completion
- **User Story 2 (Phase 4)**: Depends on Foundational completion and can follow US1
- **Polish (Phase 5)**: Depends on desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: No dependencies beyond Foundational
- **US2 (P2)**: Builds on US1 UI flow but can be implemented after Foundational

### Parallel Opportunities

- T002, T003, T004, T007, T008, T009 can run in parallel
- UI tasks T011 and T012 can run in parallel once T001 is done

---

## Parallel Example: User Story 1

```bash
Task: "Add delete action hook in src/features/stats/useBranchDelete.ts"
Task: "Add delete dialog UI with trash icon in src/features/stats/BranchDeleteDialog.tsx"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1 and Phase 2
2. Complete Phase 3 (US1)
3. Validate US1 with quickstart steps

### Incremental Delivery

1. Deliver US1 (delete flow)
2. Add US2 (impact summary and block reasons)
3. Finish polish tasks
