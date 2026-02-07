# Data Model: Project Autocomplete

## Entities

### Project

- projectId: integer (unique identifier)
- name: string (display name)
- pathWithNamespace: string (namespace path)
- webUrl: string (optional, existing contract)

### ProjectSearchState (UI state)

- query: string (current input)
- debouncedQuery: string (query after 150ms debounce)
- selectedProjectId: integer | null
- isOpen: boolean (dropdown open state)
- isLoading: boolean (project list loading state)

### ProjectSearchResult (derived)

- items: Project[] (filtered list, max 100)
- isEmpty: boolean (no matches)
- isTruncated: boolean (matches exceed 100)

## Relationships

- ProjectSearchState references Project by `selectedProjectId`.
- ProjectSearchResult is derived from Project list + ProjectSearchState.debouncedQuery.

## Validation Rules

- Case-insensitive contains match across `name` and `pathWithNamespace`.
- Accepts special characters ("-", "\_", "/") and Japanese text.
- Filtering starts at 1 character; empty input shows all projects.
- Displayed results capped at 100 with a "refine" message.

## State Transitions

- Idle -> Filtering: user types (after 150ms debounce).
- Filtering -> Results: matches found (<= 100).
- Filtering -> Truncated: matches found (> 100).
- Filtering -> Empty: no matches.
- Results/Empty/Truncated -> Selected: user selects a project (click or Enter).
- Any -> Idle: user clears input (show all).
