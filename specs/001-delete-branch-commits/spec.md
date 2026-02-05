# Feature Specification: Delete Collected Branch Commits

**Feature Branch**: `001-delete-branch-commits`  
**Created**: 2026-02-06  
**Status**: Draft  
**Input**: User description: "ユーザーとして、収集したコミットをブランチ単位で削除したい。なぜなら、間違って収集してしまったブランチを集計表示したくないから。"

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Remove miscollected branch data (Priority: P1)

As a user, I want to delete all collected commits for a specific branch so that incorrect branches no longer appear in aggregated views.

**Why this priority**: This directly addresses incorrect reporting, which undermines trust in the analytics.

**Independent Test**: Can be fully tested by deleting a target branch and confirming that all aggregates exclude that branch afterward.

**Acceptance Scenarios**:

1. **Given** collected commits exist for Branch A and Branch B, **When** I delete Branch A, **Then** Branch A commits are removed and aggregates only reflect Branch B.
2. **Given** I open the delete flow for a branch, **When** I cancel confirmation, **Then** no data is removed and aggregates remain unchanged.

---

### User Story 2 - Verify deletion impact before confirming (Priority: P2)

As a user, I want to see how many commits and which summaries will be affected before I confirm deletion so I can avoid accidental data loss.

**Why this priority**: Users need confidence and clarity before performing irreversible actions.

**Independent Test**: Can be fully tested by viewing the impact summary and completing deletion with the reported counts.

**Acceptance Scenarios**:

1. **Given** a branch with collected commits, **When** I initiate deletion, **Then** I see the commit count and affected summary indicators before confirming.

### Edge Cases

- Branch has zero collected commits.
- Branch name matches another project; deletion stays within the currently selected project context.
- A collection run is in progress for the same project and branch.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST allow users to select a project context and a branch for deletion.
- **FR-002**: System MUST show a pre-confirmation impact summary including the number of commits to be removed and which aggregate views will change.
- **FR-003**: System MUST require explicit confirmation before deleting collected commits for a branch.
- **FR-004**: System MUST remove deleted branch commits from all aggregate views and exports.
- **FR-005**: System MUST keep all non-selected branches unchanged.
- **FR-006**: System MUST show a non-destructive message when the selected branch has no collected commits.
- **FR-007**: System MUST block deletion when a collection run is active for the same project and branch, and explain the reason.

### Key Entities _(include if feature involves data)_

- **Collected Commit**: A stored commit record with attributes such as author, timestamp, project, and branch.
- **Branch**: A named branch within a project used to group collected commits.
- **Project**: The repository context that scopes branch names and aggregates.
- **Aggregate Summary**: Derived totals such as per-user and per-period commit counts.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can complete branch deletion in under 2 minutes.
- **SC-002**: Updated aggregates reflect the deletion within 10 seconds after confirmation.
- **SC-003**: At least 95% of users can complete this task without support.
- **SC-004**: Incorrect aggregation incidents related to miscollected branches drop by 80% within one month.

## Assumptions

- Deletion affects only locally collected data and does not modify remote repositories.
- Deletion scope is limited to the currently selected project and branch.
- If the user recollects the same branch later, its commits can reappear in aggregates.

## Dependencies

- None identified.

## Out of Scope

- Deleting or modifying remote branches.
- Editing or deleting individual commits within a branch.
- Bulk deletion across multiple projects in a single action.
