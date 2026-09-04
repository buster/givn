# Design Review: retire-semantic-review-guidance

## Coverage

| Question | Answer | Where |
|---|---|---|
| New components? | None — spec content change | design.md §Overview |
| Step definitions | Two files updated; removed-placeholder steps retained so the removal deltas still parse | design.md §Step Definitions |
| Coverage of removed behaviour? | The removed scenarios asserted deleted semantic guidance; replacements assert the surviving deterministic gates | proposal.md |
| E2e | One new `@e2e` scenario drives the CLI subprocess (one interaction: reviewer requests review guidance) | delta |

## Stress-test

- **Does removing the scenarios lose coverage of review guidance?** No —
  the replacements assert the deterministic disposition guidance, which is
  what survives; the semantic content is intentionally deleted by the
  retirement plan (task 1.3).
- **What about other steps files referencing semantic sections?** They
  belong to retired capabilities and no longer execute (their feature
  files are gone); they are deleted with the feature flags in task 1.2.

DESIGN-REVIEW: PASS
