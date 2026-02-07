# Research: Project Autocomplete

## Decision 1: Client-side filtering

- Decision: Filter the already-loaded project list on the client.
- Rationale: Keeps interaction fast (no extra round-trips), aligns with existing `get_projects`/`sync_projects` flow, and fits the expected scale (tens to low hundreds of projects).
- Alternatives considered: Server-side search per keystroke (rejected: added latency and extra API surface). Hybrid fallback (rejected: unnecessary complexity at current scale).

## Decision 2: Matching rule (case-insensitive contains)

- Decision: Use case-insensitive partial match against `name` and `pathWithNamespace`.
- Rationale: Matches user expectations in the spec ("ana" matches "gitlab-analyzer"), supports namespaces, and handles mixed case or Japanese names.
- Alternatives considered: Prefix-only match (rejected: too strict). Tokenized prefix match (rejected: added complexity without clear UX gain).

## Decision 3: Debounce 150ms

- Decision: Apply a 150ms debounce to input changes before filtering.
- Rationale: Smooths rapid typing without noticeable lag; keeps UI responsive with 100+ items.
- Alternatives considered: No debounce (rejected: unnecessary re-renders). 300ms+ (rejected: feels sluggish).

## Decision 4: Minimum input length

- Decision: Start filtering at 1 character; empty input shows all.
- Rationale: Immediate narrowing while preserving "clear to show all" behavior described in the scenarios.
- Alternatives considered: 2+ characters (rejected: adds friction). 3+ characters (rejected: conflicts with current UX expectations).

## Decision 5: Maximum displayed results

- Decision: Show up to 100 results and prompt to refine when exceeded.
- Rationale: Prevents overwhelming lists while still covering typical project counts.
- Alternatives considered: 50 results (rejected: too restrictive). Unlimited (rejected: long lists degrade UX).

## Decision 6: Presentation pattern

- Decision: Dropdown combobox under the input.
- Rationale: Matches the spec clarification, supports keyboard navigation and clear selection state.
- Alternatives considered: Inline suggestions or separate panel (rejected: higher cognitive load).

## Decision 7: shadcn component approach

- Decision: Use shadcn `Command` + `Popover` (combobox pattern) to build the autocomplete UI.
- Rationale: Official example exists in the shadcn registry ("combobox-popover"), aligns with UI consistency requirements, and supports keyboard navigation out of the box.
- Alternatives considered: Native `<select>` (rejected: no filtering), custom dropdown (rejected: re-implements a11y patterns).
