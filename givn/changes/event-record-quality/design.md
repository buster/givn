# Design: event-record-quality

## Decisions

- **Decides D1: mark the linked usage derived, do not omit it.** A usage entry
  emitted beside a turn keeps its counts and carries `data.derived: true`;
  its `parent` already names the turn. Old consumers keep reading it; the
  analyzer can skip derived entries by rule. Omission would break linked-usage
  consumers.
- **Decides D2: answered questions carry selected, source, and time.** The
  intake accepts `selected`, `answered_by` (alias `answer_source`),
  `answered_at`, and an optional bounded `answer`. An unanswered question has
  none of them; silence is never approval.
- **Decides D3: failed commands record a bounded first error line.** The
  recorder's `finish` receives the dispatch error, stores `data.error` through
  the existing `bound_error` first-line bound, and leaves full output opt-in.
- **D4: model identity is additive.** `provider` and `variant` join the turn,
  usage, and subagent data when reported; `model` is already present. A missing
  value stays absent, and `usage: unavailable` stays explicit.

## Architecture impact

| File | Change |
|---|---|
| `src/history.rs` | `finish` accepts an optional error and stores the bounded line in the command data. |
| `src/cli/history.rs` | New intake fields; derived usage marker; answer fields; provider/variant on turn and usage. |
| `src/main.rs` | Passes the dispatch error to the recorder. |
| `tests/steps/development_record_steps.rs` | One scenario covering all four guarantees through the real intake. |

The schema version stays `1`; all fields are additive and absent fields keep
their old meaning.

## Test design

Delta feature:
`givn/changes/event-record-quality/specs/inspect-development-history/development-record.feature`.
Steps: `tests/steps/development_record_steps.rs`. Runner:
`./run-tests.sh --non-e2e`; the scenario runs in the regular scope. Strict mode
is `.fail_on_skipped()`.

## Interaction coverage

The scenario uses the existing `history ingest` interaction and the existing
development-record E2E driver; no new consumer action.

## Visual Design Contract

- Interface: none
- Viewports: not applicable
- Design system: not applicable
