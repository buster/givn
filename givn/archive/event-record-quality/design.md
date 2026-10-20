# Design: event-record-quality

## Decisions

- **D1: mark a linked usage entry derived.** The usage entry beside a turn keeps
  its counts and carries `data.derived: true`; its parent is the turn. The
  analyzer can skip derived entries, and old consumers keep reading the entry.
- **D2: an answered question records selection, source, and time.** The intake
  accepts `selected`, `answered_by` (alias `answer_source`), `answered_at`, and
  an optional bounded `answer`. Silence records none of them.
- **D3: a failed command records a bounded first error line.** `finish` stores
  the dispatch error through the existing `bound_error` first-line bound. Full
  output remains opt-in.
- **D4: identity is additive.** `provider`, `model`, and `variant` join turn and
  usage data where reported. Missing values stay absent; `usage: unavailable`
  stays explicit.

## Architecture impact

| File | Change |
|---|---|
| `src/cli/history.rs` | Derived marker; answer fields; provider/variant on turn and usage. |
| `src/history.rs` | `finish` stores a bounded command error. |
| `src/main.rs` | Passes the dispatch error to the recorder. |
| `tests/steps/development_record_steps.rs` | Four scenarios through the real intake. |

Schema stays `1`; every field is additive.

## Test design

Delta feature:
`givn/changes/event-record-quality/specs/inspect-development-history/development-record.feature`.
Four scenarios: derived usage, answered question, bounded error, identity.
Steps: `tests/steps/development_record_steps.rs`. Runner:
`./run-tests.sh --non-e2e`; strict mode `.fail_on_skipped()`.

## Visual Design Contract

- Interface: none
- Viewports: not applicable
- Design system: not applicable
