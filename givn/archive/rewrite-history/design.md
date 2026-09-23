# Design: rewrite-history

## Overview

A new explicit command, `givn rewrite-history`, removes generated rules from
the project's reachable Git history through `git-filter-repo` while keeping
every working file. It is the only remediation for the historical
occurrences the cleanliness report names. Never automatic.

## Decisions

- Decides D1 (seed Q21): the command is `givn rewrite-history`, a dedicated
  top-level command — realized in `src/main.rs` and
  `src/cli/rewrite_history.rs`.
- Decides D2 (seed Q20 remainder): confirmation is `--yes`, a terminal
  prompt, or a piped-input refusal; the backup policy stays with
  `git-filter-repo` and is documented by its own output; givn never pushes.
- Decides D3 (seed Q20): Fossil and `versioned` refuse by name; the tool
  requirement is `git-filter-repo`, refused by name with install guidance
  when absent.

## Exact file changes

| Path | Change |
|---|---|
| `src/cli/rewrite_history.rs` | New command: policy and backend refusal, clean preflight, scan, confirmation, tool check, rewrite, verify, boundary report. |
| `src/cli/mod.rs` | Register the module. |
| `src/main.rs` | New `rewrite-history` subcommand with `--yes`. |
| `docs/arc42/05-building-block-view.md` | New `cli::rewrite_history` row. |
| `docs/arc42/08-crosscutting-concepts.md` | 8.7 names the rewrite as the remediation. |
| `givn/changes/rewrite-history/specs/rewrite-history/usecase.md` | New use-case document. |
| `givn/changes/rewrite-history/specs/rewrite-history/history-rewrite.feature` | Five delta scenarios. |
| `tests/steps/local_only_rules_steps.rs` | History-containment steps. |

## Step definitions

Existing file `tests/steps/local_only_rules_steps.rs` gains
`the reachable history contains/does not contain "<path>"`; the command runs
through the shared `the user runs` step.

## Runner and strict mode

Runner `./run-tests.sh`; strict `.fail_on_skipped()`. Single scenario:
`GIVN_FEATURES=givn/changes/rewrite-history/specs cargo test --test features_runner -- --exact "rewrite-history/history-rewrite.feature::<title>"`.

## Interaction Coverage Matrix

| Inventory entry | @e2e scenario title | Real interface | Driving mechanism |
|---|---|---|---|
| rewrite a project with historical rules | The rewrite purges reachable history and keeps the rule files | CLI | Real `givn rewrite-history --yes` subprocess against a real `git-filter-repo` |
| run it dirty | A dirty checkout is refused before any write | CLI | Real subprocess, exit code and stderr |
| run it on Fossil | The Fossil backend is refused by name | CLI | Real subprocess, stderr names the backend |

## Use-case traceability

| Contract element | Technical decision |
|---|---|
| No automatic rewrite | Only this command invokes `git-filter-repo`. |
| Clean preflight and confirmation | `dirty_entries` check; `--yes`/terminal/piped refusal. |
| Files preserved | `--invert-paths` rewrites commits, never the working tree. |
| Verified result | Re-scan of reachable history before success. |
| Personas | `developer--primary-user` (publish clean), `givn-maintainer--owner` (bounded, explicit). CLI text output, so Personas are named. |

## Version freshness

`git-filter-repo` is resolved by availability, never pinned.

## Local runnability & digital twins

None; the command shells out to Git. Archive `run` declaration unchanged.

## Coverage process boundaries

N/A.

## Arc42 impact

Chapter 5 gains the command row; chapter 8.7 names the remediation. No ADR
qualifies (the decision boundary is the SCM CLI plus explicit confirmation,
already documented as process). Recorded in `arc42.md`.

## Internal Primitives

None — the command reuses `Cleanliness`, `RulesPolicy`, and the backend
seam.
