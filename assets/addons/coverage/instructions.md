# Instruction: coverage

Produce a change-level coverage report by merging test coverage data from the
verify (and optionally verify-e2e) test runs.

## What this artifact is

The `coverage` artifact sits between `tasks` and `review` in the workflow graph.
It produces a `coverage.md` file in the change directory that compares the
change's coverage against the project baseline. This file gates review — it must
exist (with STATUS: PASS or STATUS: REGRESSION) before the change can proceed
to review.

## How to run

1. Ensure `tasks.md` is complete (all checkboxes checked).
2. Run `givn check coverage` — this executes `coverage.command` from the config.

The `coverage.command` script receives `GIVN_FEATURES` pointing at the change's
specs directory. It should:

1. Find coverage output files (lcov, junit.xml, cobertura.xml, etc.) produced
   by the last `verify.command` (and `verify.e2e_command`, if configured).
2. Merge them into a single report.
3. Compare against the project's coverage baseline (typically found in a
   project-level `coverage.md` at the repo root).
4. Exit 0 if coverage meets the baseline, non-zero on regression.
5. Print the unified report to stdout — givn captures it into `coverage.md` in
   the change directory.

## Coverage regression policy

Coverage MUST NOT be lower after the change than before. If a metric decreased,
the change is blocked unless the agent documents a technical reason in the
`coverage.md` report (e.g. "removed dead code that was counted as covered").

## Baseline maintenance

The project-level coverage baseline is updated by running `coverage.command`
without a change context. The baseline file lives at the project root as
`./coverage.md`.
