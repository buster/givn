# Spec gaps — attempt 1

Trial mode: `Dry-Run Ise`, whole-product rebuild. This file classifies
`attempt-1`; it is the only attempt. The trial revision is
`746e660b97028991ecd273595e28047f21d8262d`, recorded in
`evidence/ise/trial-revision.txt`. The spec copy is
`/tmp/givn-ise-self-rebuild-20261117T204903Z` and the manifest is
`attempt-1/manifest.yaml`.

The frozen acceptance command was:

```sh
GIVN_BIN=/tmp/givn-ise-self-rebuild-20261117T204903Z/target/release/givn \
GIVN_RESULT_FILE=$PWD/givn/changes/givn-self-rebuild-under-the-updated-rules/evidence/ise/attempt-1/e2e-result.json \
./run-tests.sh --e2e --package e2e
```

`GIVN_BIN` names the rebuilt binary. `GIVN_READY_FIXTURE` and `GIVN_ROOT`
are the checkout's test infrastructure and repository files; neither
constructs the product. The per-difference table is `attempt-1/gap-table.md`;
this file names the outcomes, the counts, and the five former
`specification defect` rows.

## Attempt counts

| Attempt | Total | Passed | Failed | Skipped | Result file | Run log |
|---|---|---|---|---|---|---|
| `attempt-1` | 120 | 33 | 87 | 0 | `attempt-1/e2e-result.json` | `attempt-1/e2e-run.log` |

## Setup failures

None. The run compiled the frozen package, started the runner with the
rebuilt binary, and completed with exit 1 because 87 scenarios failed. No
scenario was skipped and the result file matches the run log's summary.
The counts include the 115 selected `@e2e` definitions and the 5 outline
examples.

## Outcome counts

| Outcome | Rows | Wrong artifact |
|---|---|---|
| `implementation defect` | 87 | the rebuilt file in the spec copy, named per group and per row in `gap-table.md` |
| `specification defect` | 0 | none |
| `E2E test defect` | 0 | none |
| `acceptable undocumented change` | 0 | none |

Every difference received exactly one outcome before any edit. No difference
was closed as drift. The 87 `implementation defect` rows are classification
evidence; they are not repaired in this change, and the rebuilt files stay in
the spec copy.

## The five former Specification defects

The prior trial recorded five `specification defect` rows against
`givn/specification.yaml`: four `attribution_asset` surfaces (OpenCode,
Claude, omp, and Maki embedded integration text) and the product version
(`0.9.10` where the frozen test reads `0.9.20`). The declaration now names
`assets/agent-integrations/**` under `behavior` and the lockfile and
toolchain pin under `contracts`/`limits`; the copy carries all ten
integration assets and both compare runs report `match` before and after
the build (`compare-before.txt`, `compare-after.txt`).

| Former Specification defect | Attempt-1 evidence | Resolution | Residual difference |
|---|---|---|---|
| OpenCode embedded integration text | The copy holds `assets/agent-integrations/opencode/givn-attribution.js` and `givn-timeline.js`; `OpenCode prepares attribution using its selected model` passes every step. | Resolved by the declaration. | None; the scenario passed. |
| Claude embedded integration text | The copy holds `assets/agent-integrations/claude/givn-attribution.sh` and `givn-timeline.sh`; the Claude scenario passes asset setup and model discovery, then fails when the hook calls the rebuilt binary. | Resolved as a Specification defect. The residual failure is an `implementation defect`: the rebuilt `src/cli.rs` lacks the `attribution claude-pre-tool-use` subcommand. | `gap-table.md` row 20. |
| omp embedded integration text | The copy holds `assets/agent-integrations/omp/givn-attribution.js` and `givn-timeline.js`; the scenario reaches the commit and the committer identifies the LLM and harness. | Resolved as a Specification defect. The residual failure is an `implementation defect`: the rebuilt `src/misc.rs` stamp writes no LLM co-author trailer. | `gap-table.md` row 21. |
| Maki embedded integration text | The copy holds `assets/agent-integrations/maki/givn-attribution.lua`, `givn-timeline.lua`, and `plugin.toml`; the scenario loads the generated `init.lua`. | Resolved as a Specification defect. The residual failure is an `implementation defect`: the generated `init.lua` requires a module the loader cannot resolve, and the reporter path fails for the same reason. | `gap-table.md` rows 22 and 50. |
| Product version `0.9.10` versus `0.9.20` | `rebuilt-version.txt` records `givn 0.9.20` with SHA-256 `f943ab19a5e802985f153216a1cc4339be98ceafc514f5eecf7c4597d0146fb8`; the clean-room scenario's manifest-version step passes and the scenario now fails later, on the copy's undeclared-path check. | Resolved by the declaration. | None for the version; the later copy failure is `implementation defect` row 74. |

Five of five former `specification defect` rows are resolved. The final
table holds no `specification defect` row for those surfaces.

## Residual implementation defects

`gap-table.md` holds 87 rows in 20 groups. The rebuilt files named there are
the files the builder authored in the spec copy under `src/**`; none is a
declared Specification file. The largest groups are archive behavior (14),
check review and tasks (10), history JSON (6), the development report
timeline (7), run (6), guidance (6), and upgrade (6). The trial promotes
nothing: the rebuilt binary, the spec copy, and the manifest remain outside
the checkout.

## Neighbor record

`attempt-1/neighbor-record.md` records that the builder authored only the
replaced unit (24 files: 22 under `src/**`, 2 under `tests/**`), that no
declared Specification file was modified, added, or removed, that the
clean-room scan printed nothing for the three Cargo stores, and that no
rebuilt file was promoted.
