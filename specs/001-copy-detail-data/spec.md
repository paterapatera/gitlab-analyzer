# Feature Specification: Copy Detail Data

**Feature Branch**: `001-copy-detail-data`  
**Created**: 2026-02-08  
**Status**: Draft  
**Input**: User description: "ユーザーとして、集計表示画面の詳細データの内容をコピーして、スプレッドシートにペーストしたい。なぜなら、生産性のデータとして資料に使うことがあるから。ビューが、「プロジェクト別」と「横断」のどちらであっても問題なく使えること。詳細データのパネル内にある、コピーアイコンのボタンをクリックすることで、この機能を使用可能。コピーされたことがわかるメッセージを出すこと。"

## Clarifications

### Session 2026-02-08

- Q: コピー時のクリップボード形式はどれにしますか？ → A: タブ区切り（TSV、ヘッダー行あり）
- Q: フィルタや並び替えが適用されている場合、コピー対象はどれにしますか？ → A: 画面に表示されている行・順序のみをコピー
- Q: 詳細データが0行のとき、コピー操作はどうしますか？ → A: 空のヘッダー行のみをコピーする
- Q: コピー完了メッセージの表示時間はどれくらいにしますか？ → A: 2秒
- Q: コピー完了メッセージの表示位置はどこにしますか？ → A: 詳細データパネル内（上部）

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Copy detail data from the panel (Priority: P1)

As a user, I want to copy the detail data shown in the detail panel so that I can paste it into a spreadsheet for productivity reporting.

**Why this priority**: This is the core value of the request and enables immediate reuse of data.

**Independent Test**: Can be fully tested by clicking the copy icon in the detail panel and pasting into a spreadsheet to confirm rows and columns match the visible detail data.

**Acceptance Scenarios**:

1. **Given** detail data is visible in the detail panel, **When** I click the copy icon, **Then** the data is copied and can be pasted into a spreadsheet as a structured table.
2. **Given** detail data is visible, **When** I paste into a spreadsheet, **Then** the pasted content matches the same rows and columns shown in the panel.

---

### User Story 2 - Confirm copy completion (Priority: P2)

As a user, I want clear feedback after copying so I know the data is ready to paste.

**Why this priority**: Users need confirmation to avoid repeated clicks and uncertainty.

**Independent Test**: Can be fully tested by clicking the copy icon and verifying a success message appears each time copying completes.

**Acceptance Scenarios**:

1. **Given** I click the copy icon, **When** the copy completes, **Then** I see a message that the data was copied.

### Edge Cases

- Detail panel has zero rows available to copy.
- Detail panel is open in both "project" and "cross-project" views.
- Large detail data sets that take noticeable time to copy.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: System MUST provide a copy icon button within the detail data panel.
- **FR-002**: System MUST copy all rows and columns currently shown in the detail data panel.
- **FR-003**: System MUST include the same column order and headers as displayed in the detail data panel.
- **FR-004**: System MUST copy data as tab-separated values with a header row.
- **FR-005**: System MUST support copying in both the "project" view and the "cross-project" view.
- **FR-006**: System MUST display a user-visible confirmation message after a successful copy.
- **FR-006a**: System MUST keep the copy confirmation message visible for 2 seconds.
- **FR-006b**: System MUST display the copy confirmation message at the top of the detail data panel.
- **FR-007**: When the detail panel has no rows, System MUST copy a header-only row.
- **FR-008**: System MUST copy only the currently displayed rows and order when filters or sorting are applied.

### Key Entities _(include if feature involves data)_

- **Detail Data Row**: A single row of detailed metrics shown in the detail panel.
- **Detail Data Panel**: The UI container that lists detailed metrics for the current context.
- **View Mode**: The current aggregation scope, either project or cross-project.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Users can copy and paste detail data into a spreadsheet in under 1 minute.
- **SC-002**: Copy confirmation appears within 1 second of clicking the copy icon.
- **SC-003**: At least 95% of copy attempts result in a successful paste with correct row and column counts.
- **SC-004**: Users report a reduction in manual re-entry effort for productivity reporting by at least 50%.

## Assumptions

- Users have permission to access clipboard functionality in their environment.
- The detail panel already displays the complete set of columns expected for reporting.
- Spreadsheets accept pasted tabular content as rows and columns.

## Dependencies

- None identified.

## Out of Scope

- Exporting detail data to file formats.
- Customizing which columns are included in the copied output.
