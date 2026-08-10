# Instruction: coverage

Produce the project coverage report by running the configured measurement
script and merging its E2E and non-E2E reports.

## What this artifact is

The coverage addon is an archive-level gate. It produces the merged Cobertura
report configured by `coverage.output_path` and updates the README badges and
coverage section from that report.

## How to run

1. Ensure `tasks.md` is complete (all checkboxes checked).
2. Run `givn archive` — this executes `coverage.measure_command` and then
   `coverage.merge_command` from `givn/commands.yaml`.

The measurement and merge scripts receive `GIVN_FEATURES` and the configured
coverage paths through environment variables. They should:

1. The measurement command writes the non-E2E and E2E Cobertura files.
2. The merge command combines those files into `coverage.output_path`.
3. Exit non-zero when either command or report is invalid.
4. Let givn read the merged report and update the README.

## Coverage regression policy

Coverage is reviewed for completeness of gap classification, not against a
percentage threshold. The README values are descriptive output from the merged
report.

## Baseline maintenance

The project-level report is updated by running the two configured scripts
without a change context. The merged report lives at `coverage.output_path`.
