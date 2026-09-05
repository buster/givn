# Tasks: coverage-merge-union

Scenario-by-scenario RED→GREEN→REFACTOR for the two delta scenarios of
`extract-coverage-gate`. Single-scenario command (design.md):

```sh
GIVN_FEATURES=givn/changes/coverage-merge-union/specs \
  cargo test --no-default-features --features spec-retrieval \
  --test features_runner -- --name "<scenario title>"
```

- [ ] Task 1 (setup): none required — runner, strict mode, and step files exist (tests/features_runner.rs `.fail_on_skipped()`; proof pre-existing).
- [ ] Scenario: A line measured by both reports is counted once in the merged report
  - [ ] RED: run the scenario scoped; paste non-zero exit
  - [ ] GREEN: per-line data through `CoberturaReport` (`line_hits`), union merge with aggregate fallback in `merge_legacy_reports`, new fixture givens + merged-counter then-step in `tests/steps/extract_coverage_gate_steps.rs`; list production files; run scenario scoped; paste zero exit
  - [ ] REFACTOR: no behaviour change; re-run scenario; paste zero exit
  - [ ] COMMIT:
- [ ] Scenario: A line only one report measures counts once and uncovered lines are not dropped
  - [ ] RED: run the scenario scoped; paste non-zero exit
  - [ ] GREEN: no further production change expected (union covers it); run scenario scoped; paste zero exit
  - [ ] REFACTOR: re-run; paste zero exit
  - [ ] COMMIT:
- [ ] Task: repository script — merge-coverages.sh recounts root line totals from the merged tree (`package/class/lines/line` walk, distinct (filename, number), aggregate fallback), then regenerate the repository report and confirm root rate equals the class-level union rate.
  - [ ] COMMIT:
- [ ] Task: arc42 — update docs/arc42/08-crosscutting-concepts.md coverage-gate section with the union rule and the branch-summation limitation.
  - [ ] COMMIT:

DESIGN-REVIEW: PASS marker lives in design-review.md.
