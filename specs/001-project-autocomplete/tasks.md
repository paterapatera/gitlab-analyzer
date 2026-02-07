# Tasks: Project Autocomplete

**Input**: Design documents from `/specs/001-project-autocomplete/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, contracts/

**Tests**: Included. New or changed logic must be covered by tests per constitution.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Exact file paths are included in each task

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: UI primitives needed for the combobox pattern

- [x] T001 Add shadcn Command component in src/components/ui/command.tsx
- [x] T002 Add shadcn Popover component in src/components/ui/popover.tsx

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Shared filtering logic and reusable autocomplete component

- [x] T003 [P] Create project filter utility in src/features/projects/projectFilter.ts (case-insensitive contains, name+path, min length 1, max 100)
- [x] T004 [P] Add unit tests for project filter in src/test/features/projects/projectFilter.test.ts
- [x] T005 Create ProjectAutocomplete component in src/features/projects/ProjectAutocomplete.tsx (150ms debounce, dropdown, empty/truncate messages, clear input, loading state)

**Checkpoint**: Foundation ready - user story implementation can begin

---

## Phase 3: User Story 1 - Basic Project Search (Priority: P1) 🎯 MVP

**Goal**: Provide autocomplete project search in the commit collection screen

**Independent Test**: In the commit collection screen, typing a project name filters results, empty input shows all, no matches shows empty message, selection enables downstream UI.

### Implementation for User Story 1

- [x] T006 [US1] Integrate ProjectAutocomplete into src/features/projects/ProjectsPanel.tsx (use existing project list, show name+path in results)
- [ ] T007 [US1] Update project selection wiring in src/features/collect/CollectTab.tsx if props change
- [ ] T008 [P] [US1] Add ProjectAutocomplete UI tests in src/test/features/projects/ProjectAutocomplete.test.tsx (name filtering, empty message, clear shows all, selection)
- [x] T009 [P] [US1] Add ProjectsPanel tests in src/test/features/projects/ProjectsPanel.test.tsx (empty/loading states, select callback)

**Checkpoint**: User Story 1 functional and independently testable

---

## Phase 4: User Story 2 - Path-based Search (Priority: P2)

**Goal**: Provide autocomplete project search in the stats project view (path-based queries supported)

**Independent Test**: In the stats project view, typing a namespace path filters results by pathWithNamespace.

### Implementation for User Story 2

- [x] T010 [US2] Replace project select in src/features/stats/ProjectBranchSelector.tsx with ProjectAutocomplete (retain branch selection behavior)
- [x] T011 [US2] Update callers in src/features/stats/StatsFilters.tsx to match new ProjectBranchSelector props
- [ ] T012 [P] [US2] Add ProjectBranchSelector tests in src/test/features/stats/ProjectBranchSelector.test.tsx (path-based filtering, selection)

**Checkpoint**: User Story 2 functional and independently testable

---

## Phase 5: User Story 3 - Keyboard Navigation (Priority: P3)

**Goal**: Complete keyboard navigation for autocomplete (arrows, enter, escape)

**Independent Test**: Users can navigate with arrow keys, select with Enter, and close with Escape without mouse interaction.

### Implementation for User Story 3

- [x] T013 [US3] Add keyboard navigation handling in src/features/projects/ProjectAutocomplete.tsx (arrow/enter/escape behaviors)
- [ ] T014 [P] [US3] Add keyboard navigation tests in src/test/features/projects/ProjectAutocomplete.test.tsx

**Checkpoint**: User Story 3 functional and independently testable

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Documentation alignment and validation

- [ ] T015 [P] Update specs/001-project-autocomplete/quickstart.md if UI labels or steps changed during implementation
- [ ] T016 Run quickstart manual verification and record any deviations in specs/001-project-autocomplete/quickstart.md

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - blocks all user stories
- **User Stories (Phases 3-5)**: Depend on Foundational completion
- **Polish (Phase 6)**: Depends on desired user stories being complete

### User Story Dependencies

- **US1 (P1)**: Depends on Foundational - no dependencies on other stories
- **US2 (P2)**: Depends on Foundational - no dependency on US1 required
- **US3 (P3)**: Depends on Foundational - may build on shared autocomplete behavior

### Parallel Opportunities

- Phase 1 tasks can run in parallel
- Phase 2 tests (T004) can run in parallel with filter implementation (T003)
- US1 tests (T008, T009) can run in parallel with integration tasks
- US2 tests (T012) can run in parallel with integration tasks
- US3 tests (T014) can run in parallel with keyboard handling (T013)

---

## Parallel Example: User Story 1

```bash
Task: "Add ProjectAutocomplete UI tests in src/test/features/projects/ProjectAutocomplete.test.tsx"
Task: "Add ProjectsPanel tests in src/test/features/projects/ProjectsPanel.test.tsx"
```

---

## Parallel Example: User Story 2

```bash
Task: "Replace project select in src/features/stats/ProjectBranchSelector.tsx with ProjectAutocomplete"
Task: "Add ProjectBranchSelector tests in src/test/features/stats/ProjectBranchSelector.test.tsx"
```

---

## Parallel Example: User Story 3

```bash
Task: "Add keyboard navigation handling in src/features/projects/ProjectAutocomplete.tsx"
Task: "Add keyboard navigation tests in src/test/features/projects/ProjectAutocomplete.test.tsx"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational
3. Complete Phase 3: User Story 1
4. Validate User Story 1 independently

### Incremental Delivery

1. Setup + Foundational
2. US1 → validate
3. US2 → validate
4. US3 → validate
5. Polish and documentation updates
