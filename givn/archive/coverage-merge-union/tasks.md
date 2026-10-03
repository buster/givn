# Tasks: coverage-merge-union

Scenario-by-scenario RED→GREEN→REFACTOR for the two delta scenarios of
`extract-coverage-gate`. Single-scenario command (design.md):

```sh
GIVN_FEATURES=givn/changes/coverage-merge-union/specs \
  cargo test --no-default-features --features spec-retrieval \
  --test features_runner -- --name "<scenario title>"
```

- [x] Task 1 (setup): none required — runner, strict mode, and step files exist (tests/features_runner.rs `.fail_on_skipped()`; proof pre-existing).
- [x] Scenario: A line measured by both reports is counted once in the merged report
  - [x] RED: exit 1 — "merged report does not claim lines-valid=\"4\" … lines-covered=\"4\" lines-valid=\"8\"" (sum denominator)
  - [x] GREEN: production files: src/cobertura.rs (line_hits field, number attribute, line_hits(), union merge with aggregate fallback), tests/steps/extract_coverage_gate_steps.rs (legacy fixture givens, report-emitting scripts, merged-counter then-step); scenario exit 0 (4/4)
  - [x] REFACTOR: none needed; re-run exit 0
  - [x] COMMIT: `5bda3e7`
- [x] Scenario: A line only one report measures counts once and uncovered lines are not dropped
  - [x] RED: exit 1 — "merged report does not claim lines-valid=\"4\" … lines-covered=\"2\" lines-valid=\"5\"" (sum denominator)
  - [x] GREEN: covered by the same union change; scenario exit 0 (2/4)
  - [x] REFACTOR: none needed; re-run exit 0
  - [x] COMMIT: `5bda3e7` (both scenarios share one production change)
- [x] Task: repository script — merge-coverages.sh recounts root line totals from the merged tree (`package/class/lines/line` walk, distinct (filename, number) with hits summed, aggregate fallback); regenerated repository report: root 6946/16598 = 0.4185 equals the class-level union (was 12480/33196 = 0.3759).
  - [x] COMMIT: `4efd35c`
- [x] Task: arc42 — docs/arc42/08-crosscutting-concepts.md coverage-gate section states the union rule, the aggregate-only fallback, and the branch-summation limitation.
  - [x] COMMIT: `3117b4c`

DESIGN-REVIEW: PASS marker lives in design-review.md.
