# Ideation session: readability-experiment

## Phase

Givn change `add-readability-report`: proposal and specs accepted. Next
artifact: design.

## Progress

- 2026-10-19: Experiments, metrics, gate proposal, and wide historical replay
  completed.
- 2026-10-19: User chose two Givn changes: `add-readability-report` first,
  `add-readability-review-gate` second.
- 2026-10-19: Change renamed from baseline to report; proposal accepted.
- 2026-10-19: D1 answered: default-off addon `readability`.
- 2026-10-19: D2 answered: no stored baseline; on-demand report; the later gate
  uses change-relative analysis at base and head.
- 2026-10-19: D3 answered: Rust-only first; neutral analyzer contract; `syn`
  adapter first; tree-sitter later.
- 2026-10-19: Use case `assess-code-structure` and delta spec
  `readability-report.feature` written; `givn status` reports `specs-done`.
  Lint reports the expected `@wip` findings and shape advisories.
- 2026-10-19: Next: design the default-off addon, the report command, the
  neutral analyzer contract, and the `syn` adapter.

## Next artifact

`givn instructions design --change add-readability-report`, then write
`design.md` with the analyzer port and the report contract.
